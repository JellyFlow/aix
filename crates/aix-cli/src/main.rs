use anyhow::{anyhow, Result};
use chardetng::EncodingDetector;
use clap::{Parser, Subcommand};
use encoding_rs::{GB18030, UTF_16BE, UTF_16LE, UTF_8};
use ignore::WalkBuilder;
use oxipng::{optimize_from_memory, Options as OxiOptions};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;
use zip::write::FileOptions;

#[derive(Parser)]
#[command(name = "aix")]
#[command(about = "Ink AIX Package Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pack a directory into a .aix file
    Pack {
        /// Input directory to pack
        #[arg(value_name = "INPUT_DIR")]
        input_dir: PathBuf,

        /// Output .aix file path (optional, defaults to bundle.aix)
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,

        /// Enable optimization
        #[arg(short = 'O', long, default_value_t = false)]
        optimize: bool,

        /// Optimization level (1-3)
        #[arg(long, default_value_t = 2)]
        opt_level: u8,
    },
    /// List the contents of a .aix file
    #[command(alias = "ls")]
    List {
        /// Path to the .aix file
        #[arg(value_name = "AIX_FILE")]
        file: PathBuf,
    },
}

const UTF8_TEXT_EXTENSIONS: &[&str] = &["json", "js", "ink"];

#[derive(Debug)]
struct PreparedFile {
    data: Vec<u8>,
    converted_to_utf8: bool,
    optimized: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Pack {
            input_dir,
            output,
            optimize,
            opt_level,
        } => {
            let output_path = output
                .clone()
                .unwrap_or_else(|| PathBuf::from("bundle.aix"));
            pack_directory(input_dir, &output_path, *optimize, *opt_level)?;
            println!("Successfully packed {:?} to {:?}", input_dir, output_path);
        }
        Commands::List { file } => {
            list_aix(file)?;
        }
    }

    Ok(())
}

fn optimize_png(data: &[u8], level: u8) -> Result<Vec<u8>> {
    let mut options = OxiOptions::from_preset(level.clamp(0, 6));
    options.strip = oxipng::StripChunks::Safe;
    optimize_from_memory(data, &options).map_err(|e| anyhow!("oxipng error: {}", e))
}

fn optimize_jpeg(data: &[u8], level: u8) -> Result<Vec<u8>> {
    let mut compress = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    // Map opt_level (1-3) to quality (e.g., 75, 85, 95)
    let quality = match level {
        1 => 75.0,
        2 => 85.0,
        3 => 95.0,
        _ => 85.0,
    };

    // mozjpeg expects width/height. We need to decode it first to get dimensions.
    // However, the user just wants compression.
    // For JPEG, we might need a simpler way if we don't want to decode/encode.
    // But mozjpeg crate is for encoding.
    // Let's use `image` crate to decode and then mozjpeg to encode.
    let img = image::load_from_memory(data)?;
    let width = img.width() as usize;
    let height = img.height() as usize;

    compress.set_size(width, height);
    compress.set_quality(quality);
    let mut comp = compress.start_compress(Vec::new())?;

    let pixels = img.to_rgb8();
    comp.write_scanlines(&pixels)?;
    let writer = comp.finish()?;
    Ok(writer)
}

fn format_json_validation_error(
    path: &Path,
    source_text: &str,
    error: &serde_json::Error,
) -> anyhow::Error {
    let line = error.line();
    let column = error.column();

    if line == 0 || column == 0 {
        return anyhow!("Invalid JSON in {}: {}", path.display(), error);
    }

    let line_text = source_text
        .lines()
        .nth(line.saturating_sub(1))
        .unwrap_or_default()
        .trim_end_matches('\r');
    let pointer = format!("{}^", " ".repeat(column.saturating_sub(1)));

    anyhow!(
        "Invalid JSON: {}\n  at line {}, column {}\n{} | {}\n    {}\n{}",
        path.display(),
        line,
        column,
        line,
        line_text,
        pointer,
        error
    )
}

fn normalize_text_to_utf8(path: &Path, data: &[u8]) -> Result<(Vec<u8>, bool)> {
    if std::str::from_utf8(data).is_ok() {
        return Ok((data.to_vec(), false));
    }

    if data.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Err(anyhow!("Failed to decode {} as UTF-8", path.display()));
    }

    if data.starts_with(&[0xFF, 0xFE]) {
        let (decoded, _, had_errors) = UTF_16LE.decode(data);
        if had_errors {
            return Err(anyhow!(
                "Failed to convert {} from {} to UTF-8",
                path.display(),
                UTF_16LE.name()
            ));
        }
        return Ok((decoded.into_owned().into_bytes(), true));
    }

    if data.starts_with(&[0xFE, 0xFF]) {
        let (decoded, _, had_errors) = UTF_16BE.decode(data);
        if had_errors {
            return Err(anyhow!(
                "Failed to convert {} from {} to UTF-8",
                path.display(),
                UTF_16BE.name()
            ));
        }
        return Ok((decoded.into_owned().into_bytes(), true));
    }

    let (decoded, _, gb18030_had_errors) = GB18030.decode(data);
    if !gb18030_had_errors {
        return Ok((decoded.into_owned().into_bytes(), true));
    }

    let encoding = {
        let mut detector = EncodingDetector::new();
        detector.feed(data, true);
        detector.guess(None, true)
    };

    if encoding == UTF_8 {
        return Err(anyhow!("Failed to decode {} as UTF-8", path.display()));
    }

    let (decoded, _, had_errors) = encoding.decode(data);
    if had_errors {
        return Err(anyhow!(
            "Failed to convert {} from {} to UTF-8",
            path.display(),
            encoding.name()
        ));
    }

    Ok((decoded.into_owned().into_bytes(), true))
}

fn validate_and_prepare_json(path: &Path, data: &[u8], optimize: bool) -> Result<PreparedFile> {
    let (utf8_data, converted_to_utf8) = normalize_text_to_utf8(path, data)?;
    let source_text = std::str::from_utf8(&utf8_data)
        .map_err(|e| anyhow!("Failed to decode {} as UTF-8: {}", path.display(), e))?;
    let value: serde_json::Value = serde_json::from_slice(&utf8_data)
        .map_err(|e| format_json_validation_error(path, source_text, &e))?;

    let final_data = if optimize {
        serde_json::to_vec(&value)
            .map_err(|e| anyhow!("Failed to serialize JSON {}: {}", path.display(), e))?
    } else {
        utf8_data
    };

    Ok(PreparedFile {
        optimized: optimize && final_data.len() < data.len(),
        data: final_data,
        converted_to_utf8,
    })
}

fn prepare_file_for_packing(
    path: &Path,
    data: Vec<u8>,
    optimize: bool,
    opt_level: u8,
) -> Result<PreparedFile> {
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    if extension == "json" {
        return validate_and_prepare_json(path, &data, optimize);
    }

    if UTF8_TEXT_EXTENSIONS.contains(&extension.as_str()) {
        let (utf8_data, converted_to_utf8) = normalize_text_to_utf8(path, &data)?;
        return Ok(PreparedFile {
            data: utf8_data,
            converted_to_utf8,
            optimized: false,
        });
    }

    match extension.as_str() {
        "png" if optimize => {
            let optimized_data = optimize_png(&data, opt_level)?;
            let optimized = optimized_data.len() < data.len();
            Ok(PreparedFile {
                data: if optimized { optimized_data } else { data },
                converted_to_utf8: false,
                optimized,
            })
        }
        "jpg" | "jpeg" if optimize => {
            let optimized_data = optimize_jpeg(&data, opt_level)?;
            let optimized = optimized_data.len() < data.len();
            Ok(PreparedFile {
                data: if optimized { optimized_data } else { data },
                converted_to_utf8: false,
                optimized,
            })
        }
        _ => Ok(PreparedFile {
            data,
            converted_to_utf8: false,
            optimized: false,
        }),
    }
}

fn pack_directory(src_dir: &Path, dst_file: &Path, optimize: bool, opt_level: u8) -> Result<()> {
    if !src_dir.is_dir() {
        return Err(anyhow::anyhow!("Input path is not a directory"));
    }

    let file = File::create(dst_file)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    // Generate UUID v4
    let uuid = Uuid::new_v4().to_string();
    println!("Generated UUID: {}", uuid);

    // Add VERSION file to the root of the archive
    zip.start_file("VERSION", options)?;
    zip.write_all(uuid.as_bytes())?;

    // Use ignore::WalkBuilder to respect .aixignore and other ignore files
    let walker = WalkBuilder::new(src_dir)
        .add_custom_ignore_filename(".aixignore")
        .build();

    let mut total_original_size = 0;
    let mut total_optimized_size = 0;

    for result in walker {
        let entry = result?;
        let path = entry.path();

        // Compute relative path
        let name = path.strip_prefix(src_dir)?;
        let path_as_string = name.to_string_lossy().replace("\\", "/"); // normalize for zip

        if path.is_file() {
            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            let original_size = buffer.len();
            total_original_size += original_size;

            let prepared = prepare_file_for_packing(path, buffer, optimize, opt_level)?;
            let final_size = prepared.data.len();
            if prepared.converted_to_utf8 {
                println!("Converted {} to UTF-8 for packaging", path_as_string);
            }
            if prepared.optimized {
                let saved = original_size.saturating_sub(final_size);
                println!(
                    "Optimized {}: {} -> {} (saved {})",
                    path_as_string,
                    format_size(original_size as u64),
                    format_size(final_size as u64),
                    format_size(saved as u64)
                );
            } else {
                println!("Adding file: {}", path_as_string);
            }
            total_optimized_size += final_size;

            zip.start_file(path_as_string, options)?;
            zip.write_all(&prepared.data)?;
        } else if !name.as_os_str().is_empty() {
            // Only add directory if it's not the root itself
            println!("Adding dir: {}", path_as_string);
            zip.add_directory(path_as_string, options)?;
        }
    }
    zip.finish()?;

    let final_package_size = std::fs::metadata(dst_file)?.len();
    println!(
        "Package created: {:?} ({})",
        dst_file,
        format_size(final_package_size)
    );

    if optimize && total_original_size > 0 {
        let total_saved = total_original_size.saturating_sub(total_optimized_size);
        let ratio = (total_saved as f64 / total_original_size as f64) * 100.0;
        println!(
            "Optimization Summary: Total saved {} ({:.2}%)",
            format_size(total_saved as u64),
            ratio
        );
    }

    Ok(())
}

fn list_aix(file_path: &Path) -> Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let reader = aix::AixReader::new(buffer)?;

    println!("Contents of {:?}:", file_path);
    for entry in reader.list() {
        println!(
            "{}: {} (compressed: {})",
            entry.name,
            aix::format_size(entry.size),
            aix::format_size(entry.compressed_size)
        );
    }
    Ok(())
}

fn format_size(bytes: u64) -> String {
    aix::format_size(bytes)
}

#[cfg(test)]
mod tests {
    use super::{prepare_file_for_packing, validate_and_prepare_json, UTF8_TEXT_EXTENSIONS};
    use encoding_rs::GB18030;
    use std::path::Path;

    fn utf16le_with_bom(input: &str) -> Vec<u8> {
        let mut bytes = vec![0xFF, 0xFE];
        for unit in input.encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        bytes
    }

    fn gb18030_bytes(input: &str) -> Vec<u8> {
        let (encoded, _, had_errors) = GB18030.encode(input);
        assert!(!had_errors, "GB18030 encoding should succeed in tests");
        encoded.into_owned().into_iter().collect::<Vec<u8>>()
    }

    #[test]
    fn supported_utf8_text_extensions_are_expected() {
        assert_eq!(UTF8_TEXT_EXTENSIONS, ["json", "js", "ink"]);
    }

    #[test]
    fn validates_utf8_json() {
        let prepared =
            validate_and_prepare_json(Path::new("app.json"), br#"{"name":"demo"}"#, false)
                .expect("json should be valid");

        assert_eq!(prepared.data, br#"{"name":"demo"}"#);
        assert!(!prepared.converted_to_utf8);
    }

    #[test]
    fn rejects_invalid_json() {
        let error = validate_and_prepare_json(
            Path::new("app.json"),
            b"{\n  \"name\": \"demo\",\n}\n",
            false,
        )
        .expect_err("json should be invalid");

        assert!(error.to_string().contains("Invalid JSON"));
        assert!(error.to_string().contains("app.json"));
        assert!(error.to_string().contains("line 3, column 1"));
        assert!(error.to_string().contains("3 | }"));
        assert!(error.to_string().contains("^"));
    }

    #[test]
    fn converts_utf16_js_to_utf8_for_packing() {
        let bytes = utf16le_with_bom("const message = 'hello';");

        let prepared = prepare_file_for_packing(Path::new("app.js"), bytes, false, 2)
            .expect("js should be converted");

        assert_eq!(
            String::from_utf8(prepared.data).unwrap(),
            "const message = 'hello';"
        );
        assert!(prepared.converted_to_utf8);
        assert!(!prepared.optimized);
    }

    #[test]
    fn converts_utf16_json_and_minifies_when_optimized() {
        let bytes = utf16le_with_bom("{\n  \"name\": \"demo\"\n}");

        let prepared = prepare_file_for_packing(Path::new("app.json"), bytes, true, 2)
            .expect("json should be converted and minified");

        assert_eq!(
            String::from_utf8(prepared.data).unwrap(),
            r#"{"name":"demo"}"#
        );
        assert!(prepared.converted_to_utf8);
    }

    #[test]
    fn converts_gb18030_js_to_utf8_for_packing() {
        let bytes = gb18030_bytes("const text = '中文';");

        let prepared = prepare_file_for_packing(Path::new("app.js"), bytes, false, 2)
            .expect("gb18030 js should be converted");

        assert_eq!(
            String::from_utf8(prepared.data).unwrap(),
            "const text = '中文';"
        );
        assert!(prepared.converted_to_utf8);
    }

    #[test]
    fn converts_gb18030_json_to_utf8_and_validates() {
        let bytes = gb18030_bytes("{\"message\":\"中文\"}");

        let prepared = prepare_file_for_packing(Path::new("app.json"), bytes, false, 2)
            .expect("gb18030 json should be converted and validated");

        assert_eq!(
            String::from_utf8(prepared.data).unwrap(),
            "{\"message\":\"中文\"}"
        );
        assert!(prepared.converted_to_utf8);
    }
}
