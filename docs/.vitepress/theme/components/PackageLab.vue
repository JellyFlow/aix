<script setup lang="ts">
import { computed, ref } from "vue";
import type { AIX as AixClass, AixEntry } from "@yodaos-pkg/aix";

interface LabPageInfo {
  name: string;
  title?: string;
  description?: string;
  data_schema?: Record<string, unknown>;
  size: {
    width: number;
    height: number;
  };
}

interface LabTool {
  type: string;
  target: string;
  layout?: string;
  function: {
    name: string;
    description?: string;
    parameters: Record<string, unknown>;
  };
}

type SecondaryTab = "meta" | "pages" | "tools";

function formatBytes(value: number): string {
  if (value < 1024) {
    return `${value} B`;
  }

  if (value < 1024 * 1024) {
    return `${(value / 1024).toFixed(1)} KB`;
  }

  return `${(value / (1024 * 1024)).toFixed(2)} MB`;
}

const entries = ref<AixEntry[]>([]);
const version = ref<string | null>(null);
const title = ref<string | null>(null);
const pages = ref<LabPageInfo[]>([]);
const tools = ref<LabTool[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);
const selectedFile = ref<string | null>(null);
const fileContent = ref<string | null>(null);
const aixInstance = ref<AixClass | null>(null);
const currentTab = ref<SecondaryTab>("meta");

const TEXT_FILE_PATTERN =
  /\.(md|txt|json|js|ts|jsx|tsx|css|html|xml|yaml|yml|toml|ini|cfg|ink|wxml|wxss|wcss|svg)$/i;

const hasPackage = computed(() => entries.value.length > 0);
const selectedEntry = computed(() => entries.value.find((entry) => entry.name === selectedFile.value) ?? null);
const packageLabel = computed(() => title.value ?? "No package loaded");
const packageStats = computed(() => [
  { label: "Entries", value: String(entries.value.length) },
  { label: "Pages", value: String(pages.value.length) },
  { label: "Tools", value: String(tools.value.length) },
  { label: "Version", value: version.value ?? "Unknown" }
]);
const metadataRows = computed(() => [
  { label: "Title", value: title.value ?? "Untitled" },
  { label: "Version", value: version.value ?? "Unknown" },
  { label: "Entries", value: String(entries.value.length) },
  { label: "Pages", value: String(pages.value.length) },
  { label: "Tools", value: String(tools.value.length) }
]);

function resetState() {
  error.value = null;
  entries.value = [];
  version.value = null;
  title.value = null;
  pages.value = [];
  tools.value = [];
  fileContent.value = null;
  selectedFile.value = null;
}

function decodeEntryContent(aix: AixClass, fileName: string): string | null {
  const content = aix.readFile(fileName);
  const looksTextual = TEXT_FILE_PATTERN.test(fileName) || !fileName.includes(".");

  if (!looksTextual && content.includes(0)) {
    return null;
  }

  try {
    return new TextDecoder("utf-8", { fatal: true }).decode(content);
  } catch {
    return null;
  }
}

function selectInitialFile(aix: AixClass, nextEntries: AixEntry[]) {
  for (const entry of nextEntries) {
    const decoded = decodeEntryContent(aix, entry.name);
    if (decoded !== null) {
      selectedFile.value = entry.name;
      fileContent.value = decoded;
      return;
    }
  }

  selectedFile.value = nextEntries[0]?.name ?? null;
  fileContent.value = null;
}

async function loadAixModule() {
  return import("@yodaos-pkg/aix");
}

async function handleFileUpload(event: Event) {
  const input = event.target as HTMLInputElement | null;
  const file = input?.files?.[0];

  if (!file) {
    return;
  }

  loading.value = true;
  resetState();

  try {
    const { AIX } = await loadAixModule();
    const aix = await AIX.From(file);
    const nextEntries = aix.list();

    aixInstance.value = aix;
    entries.value = nextEntries;
    version.value = aix.getVersion() ?? "Unknown";
    title.value = aix.getTitle() ?? null;
    pages.value = aix.getPages() as unknown as LabPageInfo[];
    tools.value = aix.getTools() as unknown as LabTool[];
    currentTab.value = "meta";
    selectInitialFile(aix, nextEntries);
  } catch (err) {
    console.error("Error parsing AIX:", err);
    error.value = err instanceof Error ? err.message : String(err);
  } finally {
    loading.value = false;

    if (input) {
      input.value = "";
    }
  }
}

function viewFile(fileName: string) {
  if (!aixInstance.value) {
    return;
  }

  try {
    selectedFile.value = fileName;
    fileContent.value = decodeEntryContent(aixInstance.value, fileName);
  } catch (err) {
    console.error("Error reading file:", err);
    error.value = `Failed to read ${fileName}: ${String(err)}`;
  }
}

function formatTool(tool: LabTool): string {
  return JSON.stringify(
    {
      type: tool.type,
      target: tool.target,
      layout: tool.layout,
      parameters: tool.function.parameters
    },
    null,
    2
  );
}
</script>

<template>
  <div class="lab-shell">
    <section class="lab-topbar">
      <div class="lab-topbar-copy">
        <strong class="lab-topbar-title">Play</strong>
        <span class="lab-topbar-hint">Upload an `.aix` package to inspect its contents.</span>
      </div>

      <label class="lab-upload-button" for="file-input">
        <input
          id="file-input"
          type="file"
          accept=".aix"
          class="lab-hidden-input"
          @change="handleFileUpload"
        />
        <span>{{ loading ? "Reading package..." : hasPackage ? "Replace package" : "Upload package" }}</span>
      </label>
    </section>

    <div v-if="error" class="lab-error"><strong>Error:</strong> {{ error }}</div>

    <section v-if="!hasPackage" class="lab-empty-stage">
      <div class="lab-empty-stage-card">
        <strong>{{ loading ? "Reading package..." : "No package loaded" }}</strong>
      </div>
    </section>

    <template v-else>
      <section class="lab-stats">
        <article v-for="item in packageStats" :key="item.label" class="lab-stat-card">
          <span>{{ item.label }}</span>
          <strong>{{ item.value }}</strong>
        </article>
      </section>

      <section class="lab-workspace">
        <aside class="lab-sidebar">
          <div class="lab-panel-head">
            <h2>Files</h2>
            <span class="lab-count">{{ entries.length }}</span>
          </div>
          <div class="lab-file-list">
            <button
              v-for="entry in entries"
              :key="entry.name"
              :class="['lab-file-card', selectedFile === entry.name ? 'is-active' : '']"
              type="button"
              @click="viewFile(entry.name)"
            >
              <h3>{{ entry.name }}</h3>
              <div class="lab-file-meta">
                <span>{{ formatBytes(entry.size) }}</span>
                <span>compressed {{ formatBytes(entry.compressed_size) }}</span>
              </div>
            </button>
          </div>
        </aside>

        <section class="lab-preview-panel">
          <div class="lab-panel-head">
            <div>
              <h2>{{ selectedFile ?? "Preview" }}</h2>
              <p v-if="selectedEntry" class="lab-panel-subtitle">
                {{ formatBytes(selectedEntry.size) }}
              </p>
            </div>
          </div>
          <pre v-if="selectedFile && fileContent !== null" class="lab-preview-code">{{ fileContent }}</pre>
          <div v-else class="lab-empty lab-empty-compact">
            <p>
              {{
                selectedFile
                  ? "This file is not previewable as UTF-8 text."
                  : "No previewable file was selected."
              }}
            </p>
          </div>
        </section>
      </section>

      <section class="lab-secondary">
        <div class="lab-secondary-tabs" role="tablist" aria-label="Package details">
          <button
            type="button"
            class="lab-tab"
            :class="{ 'is-active': currentTab === 'meta' }"
            @click="currentTab = 'meta'"
          >
            Meta
          </button>
          <button
            type="button"
            class="lab-tab"
            :class="{ 'is-active': currentTab === 'pages' }"
            @click="currentTab = 'pages'"
          >
            Pages
          </button>
          <button
            type="button"
            class="lab-tab"
            :class="{ 'is-active': currentTab === 'tools' }"
            @click="currentTab = 'tools'"
          >
            Tools
          </button>
        </div>

        <div v-if="currentTab === 'meta'" class="lab-secondary-panel lab-meta-grid">
          <article v-for="item in metadataRows" :key="item.label" class="lab-meta-card">
            <span>{{ item.label }}</span>
            <strong>{{ item.value }}</strong>
          </article>
        </div>

        <div v-else-if="currentTab === 'pages'" class="lab-secondary-panel lab-pages-grid">
          <article v-for="page in pages" :key="page.name" class="lab-page-card">
            <div class="lab-page-head">
              <h3>{{ page.title || "Untitled page" }}</h3>
              <span class="lab-chip">{{ page.data_schema && Object.keys(page.data_schema).length > 0 ? "Schema" : "No schema" }}</span>
            </div>
            <p>{{ page.name }}</p>
            <div class="lab-page-meta">
              <span>{{ page.size.width.toFixed(0) }}w</span>
              <span>{{ page.size.height.toFixed(0) }}h</span>
            </div>
          </article>
          <div v-if="pages.length === 0" class="lab-empty lab-empty-compact">
            <p>No pages.</p>
          </div>
        </div>

        <div v-else class="lab-secondary-panel lab-tools-grid">
          <article v-for="tool in tools" :key="`${tool.function.name}-${tool.target}`" class="lab-tool-card">
            <div class="lab-tool-header">
              <h3>{{ tool.function.name }}</h3>
              <span class="lab-chip">{{ tool.target }}</span>
            </div>
            <pre class="lab-code">{{ formatTool(tool) }}</pre>
          </article>
          <div v-if="tools.length === 0" class="lab-empty lab-empty-compact">
            <p>No tools.</p>
          </div>
        </div>
      </section>
    </template>
  </div>
</template>
