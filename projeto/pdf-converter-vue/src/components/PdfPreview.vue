<template>
  <div class="preview-container" :style="{ transform: `scale(${zoom / 100})` }">
    <iframe ref="iframe" class="preview-iframe" :srcdoc="html" sandbox="allow-same-origin"></iframe>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  html: { type: String, default: '' },
  zoom: { type: Number, default: 80 }
})

const iframe = ref(null)
</script>

<style scoped>
.preview-container {
  flex: 1;
  overflow: auto;
  background: #1a1a1a;
  padding: 20px;
  transform-origin: top center;
}

.preview-iframe {
  width: 210mm;
  min-height: 297mm;
  margin: 0 auto;
  display: block;
  border: none;
  background: white;
  box-shadow: 0 8px 40px rgba(0,0,0,0.6), 0 0 0 1px rgba(255,255,255,0.08);
  border-radius: 2px;
}
</style>
