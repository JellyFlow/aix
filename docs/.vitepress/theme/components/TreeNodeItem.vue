<script setup lang="ts">
export interface TreeNodeData {
  id: string;
  name: string;
  kind: "directory" | "file";
  role: string;
  carries: string[];
  downstream: string;
  defaultExpanded?: boolean;
  children?: TreeNodeData[];
}

const props = defineProps<{
  node: TreeNodeData;
  depth: number;
  selectedId: string;
  expanded: Record<string, boolean>;
}>();

const emit = defineEmits<{
  toggle: [id: string];
  select: [node: TreeNodeData];
}>();

const isDirectory = props.node.kind === "directory";
</script>

<template>
  <li class="aix-tree-item">
    <div
      class="aix-tree-row"
      :class="{ 'is-selected': selectedId === node.id }"
      :style="{ paddingLeft: `${depth * 14 + 12}px` }"
    >
      <button
        v-if="isDirectory"
        class="aix-tree-toggle"
        type="button"
        @click="emit('toggle', node.id)"
        :aria-label="expanded[node.id] ? `Collapse ${node.name}` : `Expand ${node.name}`"
      >
        {{ expanded[node.id] ? "−" : "+" }}
      </button>
      <span v-else class="aix-tree-toggle aix-tree-toggle-placeholder"></span>

      <button class="aix-tree-button" type="button" @click="emit('select', node)">
        <span class="aix-tree-kind">{{ node.kind === "directory" ? "dir" : "file" }}</span>
        <span class="aix-tree-name">{{ node.name }}</span>
      </button>
    </div>

    <ul
      v-if="isDirectory && node.children?.length && expanded[node.id]"
      class="aix-tree-list"
    >
      <TreeNodeItem
        v-for="child in node.children"
        :key="child.id"
        :node="child"
        :depth="depth + 1"
        :selected-id="selectedId"
        :expanded="expanded"
        @toggle="emit('toggle', $event)"
        @select="emit('select', $event)"
      />
    </ul>
  </li>
</template>
