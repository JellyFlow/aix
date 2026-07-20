import React, { useCallback, useMemo, useState } from 'react';
import { AIX, type AixEntry, type PageInfo, type Tool } from '@yodaos-pkg/aix';

function formatBytes(value: number): string {
  if (value < 1024) {
    return `${value} B`;
  }
  if (value < 1024 * 1024) {
    return `${(value / 1024).toFixed(1)} KB`;
  }
  return `${(value / (1024 * 1024)).toFixed(2)} MB`;
}

function normalizeDocsRoot(baseUrl: string): string {
  if (baseUrl.includes('/playground/')) {
    return baseUrl.replace(/playground\/?$/, '');
  }
  return '/aix/';
}

export const App: React.FC = () => {
  const [entries, setEntries] = useState<AixEntry[]>([]);
  const [version, setVersion] = useState<string | null>(null);
  const [title, setTitle] = useState<string | null>(null);
  const [pages, setPages] = useState<PageInfo[]>([]);
  const [tools, setTools] = useState<Tool[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [fileContent, setFileContent] = useState<string | null>(null);
  const [aixInstance, setAixInstance] = useState<AIX | null>(null);

  const docsRoot = useMemo(() => normalizeDocsRoot(import.meta.env.BASE_URL || '/'), []);
  const formatHref = `${docsRoot}format`;
  const packagesHref = `${docsRoot}packages`;
  const docsHref = docsRoot;
  const playgroundGuideHref = `${docsRoot}playground`;

  const resetState = useCallback(() => {
    setError(null);
    setEntries([]);
    setVersion(null);
    setTitle(null);
    setPages([]);
    setTools([]);
    setFileContent(null);
    setSelectedFile(null);
  }, []);

  const handleFileUpload = useCallback(
    async (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      if (!file) return;

      setLoading(true);
      resetState();

      try {
        const aix = await AIX.From(file);
        setAixInstance(aix);

        const fileList = aix.list();
        setEntries(fileList);
        setVersion(aix.getVersion() || 'Unknown');
        setTitle(aix.getTitle() || null);
        setPages(aix.getPages());
        setTools(aix.getTools());
      } catch (err) {
        console.error('Error parsing AIX:', err);
        setError(err instanceof Error ? err.message : String(err));
      } finally {
        setLoading(false);
      }
    },
    [resetState],
  );

  const viewFile = useCallback(
    (fileName: string) => {
      if (!aixInstance) return;
      try {
        const content = aixInstance.readFile(fileName);
        const decoded = new TextDecoder().decode(content);
        setSelectedFile(fileName);
        setFileContent(decoded);
      } catch (err) {
        console.error('Error reading file:', err);
        setError(`Failed to read ${fileName}: ${err}`);
      }
    },
    [aixInstance],
  );

  const overview = useMemo(
    () => [
      {
        label: 'Entries',
        value: String(entries.length),
        description: 'Readable package files surfaced from the archive.'
      },
      {
        label: 'Pages',
        value: String(pages.length),
        description: 'Page-facing surfaces resolved from the package.'
      },
      {
        label: 'Tools',
        value: String(tools.length),
        description: 'Derived tool surfaces produced from page schema.'
      },
      {
        label: 'Version',
        value: version ?? 'Unknown',
        description: title ? `Package title: ${title}` : 'Package metadata remains inspectable.'
      }
    ],
    [entries.length, pages.length, title, tools.length, version],
  );

  return (
    <div className="lab-shell">
      <header className="lab-nav">
        <div className="lab-nav-inner">
          <div className="lab-brand">
            <a className="lab-brand-mark" href={docsHref}>
              AIX
            </a>
            <div className="lab-brand-copy">
              <strong>AIX Package Lab</strong>
              <span>Official browser surface for reading `.aix` artifacts</span>
            </div>
          </div>

          <nav className="lab-nav-links">
            <a href={formatHref}>Format</a>
            <a href={docsHref}>Docs</a>
            <a href={packagesHref}>Packages</a>
            <a href={playgroundGuideHref}>Guide</a>
            <a className="lab-link-button" href="https://github.com/jsar-project/aix">
              GitHub
            </a>
          </nav>
        </div>
      </header>

      <main className="lab-main">
        <section className="lab-hero">
          <div>
            <p className="lab-kicker">Official browser lab</p>
            <h1 className="lab-title">Inspect package structure, page schema, and derived tools in one reading flow.</h1>
            <p className="lab-lead">
              The AIX Package Lab validates what the format becomes after packaging. Upload a real
              `.aix` artifact to review entries, package metadata, pages, tool definitions, and
              previewable files without leaving the browser.
            </p>
            <div className="lab-actions">
              <a className="lab-button lab-button-dark" href={formatHref}>
                Read the format
              </a>
              <a className="lab-button lab-button-light" href={packagesHref}>
                Explore packages
              </a>
            </div>
          </div>

          <div className="lab-upload-card">
            <div>
              <p className="lab-kicker">Upload a package</p>
              <h2>Open a live `.aix` artifact</h2>
              <p>
                Start with a package file and the lab will surface its readable structure in the
                same sequence used by the docs.
              </p>
            </div>

            <label className="lab-upload-zone" htmlFor="file-input">
              <input
                id="file-input"
                type="file"
                accept=".aix"
                style={{ display: 'none' }}
                onChange={handleFileUpload}
              />
              <div>
                <strong>{loading ? 'Reading package...' : 'Drop or select an AIX package'}</strong>
                <span>
                  {loading
                    ? 'The package is being parsed and resolved into readable sections.'
                    : 'Upload a `.aix` file to surface package metadata, page info, tool definitions, and file previews.'}
                </span>
              </div>
            </label>

            <div className="lab-note">
              <strong>Reading order</strong>
              <span>Package overview, then pages and tools, then raw file inspection.</span>
            </div>
          </div>
        </section>

        {error && (
          <div className="lab-error">
            <strong>Error:</strong> {error}
          </div>
        )}

        <section className="lab-section">
          <div className="lab-section-head">
            <p className="lab-kicker">Package overview</p>
            <h2>Start with the package before moving into individual files.</h2>
            <p>
              The first layer keeps the artifact legible: counts, version metadata, title metadata,
              and the high-level shape of the package.
            </p>
          </div>

          {entries.length > 0 ? (
            <div className="lab-overview-grid">
              {overview.map((item) => (
                <article key={item.label} className="lab-overview-card">
                  <h3>{item.label}</h3>
                  <strong>{item.value}</strong>
                  <span>{item.description}</span>
                </article>
              ))}
            </div>
          ) : (
            <div className="lab-empty">
              <p>
                No package is loaded yet. Upload a `.aix` file above to populate the lab and begin
                the reading flow.
              </p>
            </div>
          )}
        </section>

        <section className="lab-section">
          <div className="lab-section-head">
            <p className="lab-kicker">Pages</p>
            <h2>Resolve page-bearing surfaces from the package.</h2>
            <p>
              Pages represent the navigable and schema-aware surfaces carried by the package.
            </p>
          </div>

          {pages.length > 0 ? (
            <div className="lab-pages-grid">
              {pages.map((page) => (
                <article key={page.name} className="lab-page-card">
                  <div>
                    <h3>{page.title || 'Untitled page'}</h3>
                    <p>{page.name}</p>
                  </div>
                  <div className="lab-page-meta">
                    <span className="lab-chip">{page.size.width.toFixed(0)}w</span>
                    <span className="lab-chip">{page.size.height.toFixed(0)}h</span>
                    <span className="lab-chip">
                      {page.data_schema && Object.keys(page.data_schema).length > 0 ? 'Schema present' : 'No schema'}
                    </span>
                  </div>
                  <p>{page.description || 'This page exposes package-defined structure without an extra description.'}</p>
                </article>
              ))}
            </div>
          ) : (
            <div className="lab-empty">
              <p>Pages appear here after the package is parsed.</p>
            </div>
          )}
        </section>

        <section className="lab-section">
          <div className="lab-section-head">
            <p className="lab-kicker">Derived tools</p>
            <h2>Schema becomes tool-facing structure.</h2>
            <p>
              These tool definitions show how package and page schema can be promoted into usable
              interface contracts.
            </p>
          </div>

          {tools.length > 0 ? (
            <div className="lab-tools-grid">
              {tools.map((tool) => (
                <article key={`${tool.function.name}-${tool.target}`} className="lab-tool-card">
                  <div className="lab-tool-header">
                    <h3>{tool.function.name}</h3>
                    <span className="lab-chip">{tool.target}</span>
                  </div>
                  <p>{tool.function.description || 'No explicit description is provided in this package.'}</p>
                  <pre className="lab-code">
                    {JSON.stringify(
                      {
                        type: tool.type,
                        target: tool.target,
                        layout: tool.layout,
                        parameters: tool.function.parameters
                      },
                      null,
                      2,
                    )}
                  </pre>
                </article>
              ))}
            </div>
          ) : (
            <div className="lab-empty">
              <p>Tool surfaces appear here when package pages resolve into tool definitions.</p>
            </div>
          )}
        </section>

        <section className="lab-section">
          <div className="lab-section-head">
            <p className="lab-kicker">Package files</p>
            <h2>Browse the underlying artifact and preview individual entries.</h2>
            <p>
              Once the package has been summarized, move downward into the concrete files that make
              up the artifact.
            </p>
          </div>

          <div className="lab-bottom-grid">
            <div className="lab-panel">
              <h3>File browser</h3>
              {entries.length > 0 ? (
                <div className="lab-file-list">
                  {entries.map((entry) => (
                    <button
                      key={entry.name}
                      className={`lab-file-card${selectedFile === entry.name ? ' is-active' : ''}`}
                      onClick={() => viewFile(entry.name)}
                      type="button"
                    >
                      <h3>{entry.name}</h3>
                      <div className="lab-file-meta">
                        <span>{formatBytes(entry.size)}</span>
                        <span>compressed {formatBytes(entry.compressed_size)}</span>
                      </div>
                    </button>
                  ))}
                </div>
              ) : (
                <div className="lab-empty">
                  <p>Upload a package to populate the file browser.</p>
                </div>
              )}
            </div>

            <div className="lab-preview-card">
              <h3>{selectedFile ? `Preview · ${selectedFile}` : 'File preview'}</h3>
              <p>
                {selectedFile
                  ? 'Decoded file content appears here when the selected entry is previewable as text.'
                  : 'Select a file from the browser to inspect its contents.'}
              </p>
              {selectedFile ? <pre>{fileContent}</pre> : null}
            </div>
          </div>
        </section>

        <footer className="lab-footer">
          <div className="lab-footer-card">
            <div>
              <h3>Read the model, then inspect the artifact.</h3>
              <p>
                AIX keeps the package close to its schema and tool semantics. The docs define the
                model, and the package lab lets you validate it against real files.
              </p>
            </div>
            <div className="lab-footer-links">
              <a className="lab-button lab-button-light" href={docsHref}>
                Docs
              </a>
              <a className="lab-button lab-button-light" href={formatHref}>
                Format
              </a>
              <a className="lab-button lab-button-light" href="https://github.com/jsar-project/aix">
                GitHub
              </a>
            </div>
          </div>
        </footer>
      </main>
    </div>
  );
};
