# @yodaos-pkg/aix

`@yodaos-pkg/aix` は、AIX 成果物の読み取り、作成、最適化、検査を行うためのブラウザー向け WebAssembly / TypeScript パッケージです。WebAssembly をサポートする最新ブラウザーなどの環境で動作します。

## インストール

~~~bash
npm install @yodaos-pkg/aix
~~~

~~~typescript
import { AIX } from '@yodaos-pkg/aix';

const response = await fetch('/agents/example.aix');
const aix = await AIX.From(new Uint8Array(await response.arrayBuffer()));
console.log(aix.getTitle(), aix.getPages(), aix.getTools());
~~~

`AIX.From` は WASM ランタイムを初期化し、完全なアーカイブのバイト列またはブラウザーの `File` オブジェクトを受け取ります。

## API リファレンス

### AIX.From(data)

~~~typescript
static From(data: Uint8Array | File): Promise<AIX>
~~~

完全な AIX アーカイブからインスタンスを作成します。ZIP、エントリーパス、または正規化後のエントリーが不正な場合、Promise は reject されます。

### AIX.pack(files, options?)

~~~typescript
static pack(files: AixInputFile[], options?: PackOptions): Promise<PackResult>
~~~

正規化済みのメモリー内エントリーをパッケージ化します。入力には `app.json` が必要で、`VERSION` および予約済みの `META-INF/aix` パスを含めてはいけません。

### AIX.packFromSource(files, options?)

~~~typescript
static packFromSource(files: AixInputFile[], options?: PackFromSourceOptions): Promise<PackResult>
~~~

ソースツリーをパッケージ化し、パスの正規化、入れ子になった `.aixignore` ルールの適用、ignore ファイルの除外を行います。これらの処理が完了している場合は `pack` を使用します。

### AIX.packFromFiles(files, options?)

~~~typescript
static packFromFiles(files: File[], options?: PackFromSourceOptions): Promise<PackResult>
~~~

利用可能な場合は `webkitRelativePath` を使ってブラウザーの `File` をソースエントリーに変換し、`packFromSource` に委譲します。

### AIX.optimize(data, options?)

~~~typescript
static optimize(data: Uint8Array | File, options?: OptimizeOptions): Promise<PackResult>
~~~

既存パッケージの最適化済みコピーを書き出します。最適化によりバイト列が変わるため、既存の署名は保持されません。

### インスタンスメソッド

~~~typescript
list(): AixEntry[]
readFile(name: string): Uint8Array
getVersion(): string | undefined
supportsEngine(currentVersion: string): boolean
getTitle(): string | undefined
getPages(): PageInfo[]
getWidgets(locale?: string): WidgetInfo[]
getTools(): Tool[]
~~~

`list` は正規化済みの名前と圧縮前後のサイズを返します。`readFile` は検証済みの非圧縮バイト列を返します。`getVersion` は `VERSION` を、`getTitle` は `app.json.window.navigationBarTitleText` を読み取ります。`supportsEngine` は Manifest の semver 範囲を検査します。`getPages` は Ink または複数ファイルのページを解決します。`getWidgets` は Widget の Ink エントリーを検証し、任意でロケールオーバーレイを適用します。`getTools` は OpenAI 互換の関数レコードを生成します。ページ、Schema、レイアウト、ツールの意味は[仕様](/spec?lang=ja)を参照してください。

## 型リファレンス

~~~typescript
interface AixInputFile { path: string; data: Uint8Array }

interface PackOptions {
  buildId?: string;
  engine?: string;
  optimize?: false | OptimizeOptions;
}

interface OptimizeOptions {
  level?: 1 | 2 | 3;
  json?: boolean;
  png?: boolean;
  jpeg?: boolean;
}

interface PackFromSourceOptions extends PackOptions {
  onProgress?: (event: PackProgressEvent) => void;
}

interface PackResult {
  data: Uint8Array;
  report: OptimizeReport;
  warnings: string[];
}

interface OptimizeReport {
  files: FileOptimizeReport[];
  original_size: number;
  output_size: number;
  saved_bytes: number;
}

interface FileOptimizeReport {
  path: string;
  status: 'optimized' | 'unchanged' | 'skipped';
  original_size: number;
  output_size: number;
  saved_bytes: number;
  converted_to_utf8: boolean;
}
~~~

### PackProgressEvent

~~~typescript
type PackProgressEvent =
  | { type: 'transferring_files_to_wasm' }
  | { type: 'collecting_source_inputs' }
  | { type: 'resolving_engine' }
  | { type: 'preparing_files' }
  | { type: 'file_finished'; report: FileOptimizeReport }
  | { type: 'finalizing_archive' };
~~~

### パッケージおよび派生モデル

~~~typescript
interface AixEntry { name: string; size: number; compressed_size: number }

interface PageInfo {
  name: string;
  title?: string;
  data_schema: unknown;
}

interface WidgetInfo {
  path: string;
  family: string;
  placement: 'persistent' | 'overlay';
  displayName?: string | null;
  description?: string | null;
}

interface Tool {
  type: string;
  function: { name: string; description?: string; parameters: unknown };
}
~~~

## ブラウザーでの使用例

~~~typescript
const input = document.querySelector<HTMLInputElement>('#package')!;
input.addEventListener('change', async () => {
  const file = input.files?.[0];
  if (!file) return;
  const aix = await AIX.From(file);
  console.log(aix.getVersion());
  console.table(aix.list());
  console.log(aix.getPages(), aix.getTools());
});
~~~

## Package Lab

公式のブラウザー向け Lab は[プレイ](/play?lang=ja)で利用できます。成果物のタイトル、バージョン、ページ、生成されたツール、アーカイブエントリー、生のエントリー内容を表示します。

## ソースからのビルド

~~~bash
cd crates/aix-web
npm install
npm run build
~~~

生成された配布物は `crates/aix-web/dist` に書き出されます。アプリケーションは生成途中の内部ファイルではなく、公開済みパッケージを利用してください。

## ライセンス

MIT
