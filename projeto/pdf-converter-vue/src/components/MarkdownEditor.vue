<template>
  <textarea
    ref="textarea"
    class="editor"
    :value="modelValue"
    @input="$emit('update:modelValue', $event.target.value)"
    @keyup="emitCursor"
    @click="emitCursor"
    spellcheck="false"
  ></textarea>
</template>

<script setup>
import { ref } from 'vue'

defineProps({ modelValue: String })
const emit = defineEmits(['update:modelValue', 'cursor'])

const textarea = ref(null)

const emitCursor = () => {
  const el = textarea.value
  if (!el) return
  const text = el.value.substring(0, el.selectionStart)
  const lines = text.split('\n')
  emit('cursor', { line: lines.length, col: lines[lines.length - 1].length + 1 })
}
</script>

<style scoped>
.editor {
  flex: 1;
  width: 100%;
  padding: 16px;
  background: var(--bg-primary);
  border: none;
  color: var(--text-primary);
  font-family: 'JetBrains Mono', monospace;
  font-size: 12px;
  line-height: 1.6;
  resize: none;
  outline: none;
}
.editor::selection { background: #166534; }
</style>
