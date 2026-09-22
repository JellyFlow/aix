<script setup lang="ts">
import { withBase } from "vitepress";
import PackageTreeDemo from "./PackageTreeDemo.vue";

const specHref = withBase("/spec?lang=ja");
const apiHref = withBase("/api?lang=ja");
const playHref = withBase("/play?lang=ja");
const layers = [
  ["パッケージ構造", "AIX はパッケージを不透明なバイナリではなく、読み取れる具体的なエントリーツリーとして保持します。"],
  ["ページメタデータ", "アプリとページの設定がナビゲーション可能な画面と、内容を解釈するためのコンテキストを定義します。"],
  ["Schema からツールへ", "Schema を持つページは、元のパッケージとの関連を保ったままツール向け契約に変換できます。"]
];
const flow = [
  ["01", "アーカイブを開く", "パッケージ境界から始め、VERSION、app.json、ページファイル、アセットを列挙します。"],
  ["02", "ページを解決する", "ページ設定と単一ファイルコンポーネントを、概要、説明、レイアウト情報へ変換します。"],
  ["03", "Schema を解釈する", "Schema が明確なデータ契約を加え、パッケージを単なるファイルコンテナ以上のものにします。"],
  ["04", "ツールを生成する", "別の場所で再構築せず、パッケージ固有の構造からツール記述を生成します。"]
];
const packages = [
  { kicker: "コア形式", definition: "aix crate は AIX 形式、パッケージ読み取りモデル、ページ解析、構造上の意味を定義します。", usage: "Rust で .aix を読み取り、ページメタデータを解決し、形式固有の構造を生成するときに使用します。", command: "crates/aix", meta: [["Crate", "crates/aix"], ["提供", "形式定義、パッケージ読み取り、ページ解析"], ["依存", "ワークスペース基盤のみ"]] },
  { kicker: "npm CLI", definition: "@yodaos-pkg/aix-cli は形式モデルをパッケージ化、検証、検査のワークフローに変換します。", usage: "自動化スクリプト、開発環境、リリース工程から AIX を扱うときに使用します。", command: "npm install -g @yodaos-pkg/aix-cli", meta: [["パッケージ", "@yodaos-pkg/aix-cli"], ["提供", "検査、パッケージ化、検証、CLI ワークフロー"], ["依存", "@yodaos-pkg/aix"]] },
  { kicker: "Web API", definition: "@yodaos-pkg/aix は WASM を通して同じ形式機能をブラウザーに提供します。", usage: "統一された形式モデルのまま、ブラウザーで AIX を読み取り、実演、統合できます。", command: "npm install @yodaos-pkg/aix", meta: [["パッケージ", "@yodaos-pkg/aix"], ["提供", "WASM バインディング、ブラウザー検査、Web ツール"], ["依存", "AIX コアと Web バインディング"]] }
];
</script>

<template>
  <main class="aix-doc-home">
    <section class="aix-doc-hero"><div class="aix-doc-container aix-doc-hero-grid"><div class="aix-doc-hero-copy"><p class="aix-doc-kicker">AIX ファイル形式</p><h1 class="aix-doc-hero-title">AIX は AI のための実行可能パッケージ形式です。</h1><p class="aix-doc-hero-lead">ページ、Schema、ツールを AI Agent 向けの配布可能な成果物にまとめます。</p><div class="aix-doc-hero-actions"><a class="aix-doc-button aix-doc-button-dark" :href="specHref">仕様を読む</a><a class="aix-doc-button aix-doc-button-light" :href="playHref">AIX を試す</a></div></div><PackageTreeDemo locale="ja" /></div></section>
    <section class="aix-doc-install-section"><div class="aix-doc-container"><div class="aix-doc-install-shell"><div class="aix-doc-install" aria-label="CLI インストールコマンド"><p class="aix-doc-install-title">CLI から始めましょう。</p><div class="aix-doc-install-command-row"><pre class="aix-doc-install-command"><code>npm install -g @yodaos-pkg/aix-cli</code></pre></div></div></div></div></section>
    <section class="aix-doc-section"><div class="aix-doc-container"><div class="aix-doc-section-head"><p class="aix-doc-kicker">読み取りモデル</p><h2>最初から読みやすく。</h2><p>パッケージの内容、ファイルによるページ定義、Schema からランタイム画面への展開を示します。</p></div><div class="aix-doc-grid aix-doc-grid-3"><article v-for="item in layers" :key="item[0]" class="aix-doc-card"><h3>{{ item[0] }}</h3><p>{{ item[1] }}</p></article></div></div></section>
    <section class="aix-doc-section aix-doc-section-compact"><div class="aix-doc-container"><div class="aix-doc-section-head"><p class="aix-doc-kicker">AIX の解釈手順</p><h2>パッケージからツールへ。</h2></div><div class="aix-doc-flow"><article v-for="item in flow" :key="item[0]" class="aix-doc-step"><span class="aix-doc-step-number">{{ item[0] }}</span><div><h3>{{ item[1] }}</h3><p>{{ item[2] }}</p></div></article></div></div></section>
    <section v-for="item in packages" :key="item.kicker" class="aix-doc-section aix-doc-package-section"><div class="aix-doc-container"><div class="aix-doc-package-grid"><div class="aix-doc-section-head aix-doc-package-copy"><p class="aix-doc-kicker">{{ item.kicker }}</p><div class="aix-doc-prose"><p>{{ item.definition }}</p><p>{{ item.usage }}</p></div><div class="aix-doc-command-wrap"><span class="aix-doc-command-label">クイックスタート</span><pre class="aix-doc-command"><code>{{ item.command }}</code></pre></div></div><aside class="aix-doc-package-meta" aria-label="パッケージメタデータ"><dl class="aix-doc-package-meta-list"><div v-for="meta in item.meta" :key="meta[0]" class="aix-doc-package-meta-row"><dt>{{ meta[0] }}</dt><dd>{{ meta[1] }}</dd></div></dl></aside></div></div></section>
    <section class="aix-doc-section aix-doc-final"><div class="aix-doc-container aix-doc-final-grid"><p class="aix-doc-kicker">次のステップ</p><div class="aix-doc-hero-actions"><a class="aix-doc-button aix-doc-button-dark" :href="apiHref">API を見る</a><a class="aix-doc-button aix-doc-button-light" :href="playHref">AIX を試す</a></div></div></section>
  </main>
</template>
