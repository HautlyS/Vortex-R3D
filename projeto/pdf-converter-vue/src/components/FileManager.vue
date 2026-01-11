<template>
  <div class="file-manager">
    <div class="header">◈ ARQUIVOS</div>
    <div class="file-list">
      <div
        v-for="file in files"
        :key="file.name"
        class="file-item"
        :class="{ active: selected === file.name }"
        @click="$emit('select', file)"
      >
        {{ file.name }}
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  files: { type: Array, default: () => [] },
  selected: String
})
defineEmits(['select'])
</script>

<style scoped>
.file-manager { padding: 12px; }
.header {
  font-size: 10px; font-weight: 700;
  color: var(--accent);
  letter-spacing: 1px;
  margin-bottom: 12px;
}
.file-list { display: flex; flex-direction: column; gap: 4px; }
.file-item {
  padding: 10px 12px;
  border-radius: 6px;
  font-size: 11px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s;
}
.file-item:hover { background: var(--bg-tertiary); color: var(--text-primary); }
.file-item.active { background: #166534; color: white; }
</style>
