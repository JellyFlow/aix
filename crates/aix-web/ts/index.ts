import init, { AixReaderWasm } from '../dist/pkg/aix_web';

export interface AixEntry {
  name: string;
  size: number;
  compressed_size: number;
}

export interface PageInfo {
  name: string;
  title?: string;
  data_schema: any;
}

export interface Tool {
  type: string;
  function: {
    name: string;
    description?: string;
    parameters: any;
  };
}

export class AIX {
  private reader: AixReaderWasm;

  private constructor(reader: AixReaderWasm) {
    this.reader = reader;
  }

  /**
   * Initialize the WASM module and create an AIX instance from the given data.
   * @param data The .aix file content as Uint8Array or File
   */
  static async From(data: Uint8Array | File): Promise<AIX> {
    await init();
    let buffer: Uint8Array;
    if (data instanceof Uint8Array) {
      buffer = data;
    } else {
      const arrayBuffer = await data.arrayBuffer();
      buffer = new Uint8Array(arrayBuffer);
    }
    const reader = new AixReaderWasm(buffer);
    return new AIX(reader);
  }

  /**
   * List all files in the AIX package.
   */
  list(): AixEntry[] {
    return this.reader.list() as AixEntry[];
  }

  /**
   * Read the content of a file from the AIX package.
   * @param name The name of the file
   */
  readFile(name: string): Uint8Array {
    return this.reader.read_file(name);
  }

  /**
   * Get the version metadata from the AIX package.
   */
  getVersion(): string | undefined {
    return this.reader.get_version();
  }

  /**
   * Get the title from app.json.
   */
  getTitle(): string | undefined {
    return (this.reader as any).get_title();
  }

  /**
   * Get all pages from app.json and pages/*.json.
   */
  getPages(): PageInfo[] {
    return (this.reader as any).get_pages() as PageInfo[];
  }

  /**
   * Get all tools from app.json and pages/*.json in OpenAI format.
   */
  getTools(): Tool[] {
    return (this.reader as any).get_tools() as Tool[];
  }
}
