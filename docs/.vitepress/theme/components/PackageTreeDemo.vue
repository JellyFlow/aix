<script setup lang="ts">
import { computed, reactive, ref } from "vue";
import TreeNodeItem, { type TreeNodeData } from "./TreeNodeItem.vue";

const tree: TreeNodeData = {
  id: "root",
  name: "example.aix",
  kind: "directory",
  role: "The packaged artifact itself. Readers start here and then walk into metadata, pages, and assets.",
  carries: ["Archive entries", "Directory hierarchy", "Packaged application boundary"],
  downstream: "Everything resolved by `aix`, `aix-cli`, and `aix-web` begins from this root artifact.",
  defaultExpanded: true,
  children: [
    {
      id: "version",
      name: "VERSION",
      kind: "file",
      role: "Carries the package version identifier generated during packaging.",
      carries: ["Version UUID", "Package identity"],
      downstream: "Shown by package readers as top-level package metadata."
    },
    {
      id: "app-json",
      name: "app.json",
      kind: "file",
      role: "Declares app-level configuration and the page list that defines the package surface.",
      carries: ["Page routes", "Window config", "Top-level package intent"],
      downstream: "Used to resolve the package title and the list of pages to inspect."
    },
    {
      id: "pages",
      name: "pages",
      kind: "directory",
      role: "Contains page-level files that define UI structure, metadata, and schema-bearing surfaces.",
      carries: ["Page config", "Layout files", "Schema sources"],
      downstream: "Page readers walk this subtree to derive page info and tool-facing contracts.",
      defaultExpanded: true,
      children: [
        {
          id: "index",
          name: "index",
          kind: "directory",
          role: "Represents a traditional multi-file page entry.",
          carries: ["Metadata", "Template", "Style"],
          downstream: "Combined into page summaries and layout constraints.",
          defaultExpanded: true,
          children: [
            {
              id: "index-json",
              name: "index.json",
              kind: "file",
              role: "Page metadata and schema source for the `index` page.",
              carries: ["navigationBarTitleText", "schema.data", "description"],
              downstream: "Used to derive page titles and tool parameters."
            },
            {
              id: "index-wxml",
              name: "index.wxml",
              kind: "file",
              role: "Template markup for the page.",
              carries: ["Layout structure", "Element tree"],
              downstream: "Feeds the page analyzer when layout constraints are computed."
            },
            {
              id: "index-wxss",
              name: "index.wxss",
              kind: "file",
              role: "Style layer for the page.",
              carries: ["Sizing", "Presentation rules"],
              downstream: "Combined with markup to estimate page width and height."
            }
          ]
        },
        {
          id: "detail-ink",
          name: "detail.ink",
          kind: "file",
          role: "Single-file component variant that can carry config, template, and style together.",
          carries: ["Inline config", "Template", "Style"],
          downstream: "Parsed as one source and still resolved into page info and layout data."
        }
      ]
    },
    {
      id: "assets",
      name: "assets",
      kind: "directory",
      role: "Static resources referenced by the packaged application.",
      carries: ["Icons", "Illustrations", "Other bundled assets"],
      downstream: "Packaged as readable entries and surfaced by file browsers in the lab.",
      children: [
        {
          id: "icon-svg",
          name: "icon.svg",
          kind: "file",
          role: "Example static asset bundled into the package.",
          carries: ["Static resource bytes"],
          downstream: "Can be listed and previewed as a concrete package entry."
        }
      ]
    }
  ]
};

function collectDefaults(node: TreeNodeData, target: Record<string, boolean>) {
  target[node.id] = Boolean(node.defaultExpanded);
  node.children?.forEach((child) => collectDefaults(child, target));
}

function flatten(node: TreeNodeData): TreeNodeData[] {
  return [node, ...(node.children?.flatMap(flatten) ?? [])];
}

const expanded = reactive<Record<string, boolean>>({});
collectDefaults(tree, expanded);

const allNodes = flatten(tree);
const selectedId = ref("app-json");

const selectedNode = computed(
  () => allNodes.find((node) => node.id === selectedId.value) ?? tree
);

function toggleNode(id: string) {
  expanded[id] = !expanded[id];
}

function selectNode(node: TreeNodeData) {
  selectedId.value = node.id;
}
</script>

<template>
  <div class="aix-tree-demo">
    <div class="aix-tree-shell">
      <div class="aix-tree-header">
        <strong>Package Structure Demo</strong>
        <span>Interactive example of how an `.aix` artifact stays readable.</span>
      </div>

      <div class="aix-tree-layout">
        <div class="aix-tree-browser">
          <ul class="aix-tree-list">
            <TreeNodeItem
              :node="tree"
              :depth="0"
              :selected-id="selectedId"
              :expanded="expanded"
              @toggle="toggleNode"
              @select="selectNode"
            />
          </ul>
        </div>

        <aside class="aix-tree-detail">
          <p class="aix-tree-detail-label">Selected node</p>
          <h3>{{ selectedNode.name }}</h3>
          <p>{{ selectedNode.role }}</p>

          <div class="aix-tree-detail-block">
            <strong>Carries</strong>
            <ul>
              <li v-for="item in selectedNode.carries" :key="item">{{ item }}</li>
            </ul>
          </div>

          <div class="aix-tree-detail-block">
            <strong>Downstream usage</strong>
            <p>{{ selectedNode.downstream }}</p>
          </div>
        </aside>
      </div>
    </div>
  </div>
</template>
