import React, { useState, useCallback } from 'react';
import { AIX, type AixEntry, type PageInfo, type Tool } from '@yodaos-pkg/aix';

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

  const handleFileUpload = useCallback(async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    setLoading(true);
    setError(null);
    setEntries([]);
    setVersion(null);
    setTitle(null);
    setPages([]);
    setTools([]);
    setFileContent(null);
    setSelectedFile(null);

    try {
      const aix = await AIX.From(file);
      setAixInstance(aix);

      const fileList = aix.list();
      setEntries(fileList);

      const ver = aix.getVersion();
      setVersion(ver || 'Unknown');

      const appTitle = aix.getTitle();
      setTitle(appTitle || null);

      const pageList = aix.getPages();
      console.info(pageList);
      setPages(pageList);

      const toolList = aix.getTools();
      console.info(toolList);
      setTools(toolList);
    } catch (err) {
      console.error('Error parsing AIX:', err);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

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

  return (
    <div className="container">
      <h1>AIX Web Tester</h1>

      <div className="upload-area" onClick={() => document.getElementById('file-input')?.click()}>
        <input
          id="file-input"
          type="file"
          accept=".aix"
          style={{ display: 'none' }}
          onChange={handleFileUpload}
        />
        {loading ? (
          <p>Processing AIX package...</p>
        ) : (
          <p>
            Click or drag an <strong>.aix</strong> file here to test
          </p>
        )}
      </div>

      {error && (
        <div
          style={{
            color: '#e74c3c',
            background: '#fdedec',
            padding: '1rem',
            borderRadius: '4px',
            marginBottom: '1rem',
          }}
        >
          <strong>Error:</strong> {error}
        </div>
      )}

      {version && (
        <div style={{ marginBottom: '1rem', display: 'flex', gap: '1rem', alignItems: 'center' }}>
          <div>
            <strong>Version:</strong> <span className="version-badge">{version}</span>
          </div>
          {title && (
            <div>
              <strong>Title:</strong>{' '}
              <span className="version-badge" style={{ background: '#2ecc71' }}>
                {title}
              </span>
            </div>
          )}
        </div>
      )}

      {pages.length > 0 && (
        <div style={{ marginBottom: '2rem' }}>
          <h3>Pages</h3>
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))',
              gap: '1rem',
            }}
          >
            {pages.map((page, idx) => (
              <div
                key={idx}
                style={{
                  background: '#fff',
                  padding: '1rem',
                  borderRadius: '8px',
                  border: '1px solid #ddd',
                }}
              >
                <div style={{ fontWeight: 'bold' }}>{page.title || 'Untitled'}</div>
                <div style={{ fontSize: '0.8rem', color: '#666' }}>{page.name}</div>
              </div>
            ))}
          </div>
        </div>
      )}

      {tools.length > 0 && (
        <div style={{ marginBottom: '2rem' }}>
          <h3>OpenAI Tools</h3>
          <pre style={{ maxHeight: '300px', fontSize: '0.8rem' }}>
            {JSON.stringify(tools, null, 2)}
          </pre>
        </div>
      )}

      {entries.length > 0 && (
        <div>
          <h3>Package Contents ({entries.length} files)</h3>
          <table>
            <thead>
              <tr>
                <th>File Name</th>
                <th>Size</th>
                <th>Compressed</th>
                <th>Action</th>
              </tr>
            </thead>
            <tbody>
              {entries.map((entry, idx) => (
                <tr key={idx}>
                  <td>{entry.name}</td>
                  <td>{(entry.size / 1024).toFixed(2)} KB</td>
                  <td>{(entry.compressed_size / 1024).toFixed(2)} KB</td>
                  <td>
                    <button onClick={() => viewFile(entry.name)}>View</button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {selectedFile && (
        <div style={{ marginTop: '2rem' }}>
          <h3>Preview: {selectedFile}</h3>
          <pre>{fileContent}</pre>
        </div>
      )}
    </div>
  );
};
