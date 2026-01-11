<template>
  <div class="app">
    <header class="header">
      <div class="header-left">
        <div class="logo">
          <span class="logo-icon">⚡</span>
          <span class="logo-text">TECHNO SUTRA</span>
          <span class="logo-version">PDF v6.0</span>
        </div>
      </div>
      
      <div class="header-center">
        <ThemeSelector v-model="currentTheme" />
      </div>
      
      <div class="header-right">
        <button class="btn btn-ghost" @click="refreshFiles" title="Refresh">🔄</button>
        <button class="btn btn-primary" @click="exportPDF" :disabled="!markdown || isProcessing">
          <span class="btn-icon">📄</span> Export PDF
        </button>
        <button class="btn btn-accent" @click="exportAllPDFs" :disabled="files.length === 0 || isProcessing">
          <span class="btn-icon">📚</span> All ({{ files.length }})
        </button>
        <div class="status-badge" :class="statusClass">
          <span class="status-dot"></span>
          {{ status }}
        </div>
      </div>
    </header>
    
    <main class="main">
      <aside class="sidebar">
        <div class="sidebar-header">
          <span class="sidebar-title">◈ ARQUIVOS</span>
          <span class="file-count">{{ files.length }}</span>
        </div>
        <div class="file-list">
          <div
            v-for="file in files"
            :key="file.name"
            class="file-item"
            :class="{ active: currentFile === file.name }"
            @click="loadFile(file)"
          >
            <span class="file-icon">📄</span>
            <span class="file-name">{{ file.name }}</span>
          </div>
        </div>
      </aside>
      
      <section class="editor-panel" :style="{ flex: `0 0 ${editorWidth}px` }">
        <div class="panel-header">
          <div class="panel-title">
            <span class="panel-icon">◉</span>
            <span>{{ currentFile || 'EDITOR' }}</span>
          </div>
          <div class="panel-info">
            <span class="cursor-pos">L:{{ cursorLine }} C:{{ cursorCol }}</span>
          </div>
        </div>
        <textarea
          class="editor"
          v-model="markdown"
          @keyup="updateCursor"
          @click="updateCursor"
          spellcheck="false"
          placeholder="Select a file..."
        ></textarea>
      </section>
      
      <div class="resizer" @mousedown="startResize"></div>
      
      <section class="preview-panel">
        <div class="panel-header preview-header">
          <div class="panel-title">
            <span class="panel-icon">◉</span>
            <span>PREVIEW</span>
            <span class="page-info" v-if="pageCount > 0">{{ pageCount }} pg</span>
          </div>
          <div class="zoom-control">
            <button class="zoom-btn" @click="zoom = Math.max(30, zoom - 10)">−</button>
            <span class="zoom-value">{{ zoom }}%</span>
            <button class="zoom-btn" @click="zoom = Math.min(150, zoom + 10)">+</button>
          </div>
        </div>
        <div class="preview-scroll">
          <div class="preview-pages" :style="{ transform: `scale(${zoom / 100})`, transformOrigin: 'top center' }">
            <div v-for="(page, idx) in renderedPages" :key="idx" class="pdf-page" v-html="page"></div>
            <div v-if="renderedPages.length === 0" class="empty-preview">Select a file</div>
          </div>
        </div>
      </section>
    </main>
    
    <div v-if="batchProgress.active" class="modal-overlay">
      <div class="modal">
        <div class="modal-header">📚 Batch Export</div>
        <div class="modal-body">
          <div class="progress-info">{{ batchProgress.current }} / {{ batchProgress.total }}</div>
          <div class="progress-file">{{ batchProgress.currentFile }}</div>
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: `${(batchProgress.current / batchProgress.total) * 100}%` }"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { marked } from 'marked'
import { getCSS } from './themes/index.js'
import { splitIntoPages, generatePageHTMLArray, generatePDFPages } from './utils/pageSplitter.js'
import ThemeSelector from './components/ThemeSelector.vue'

marked.setOptions({ gfm: true, breaks: true })

const currentTheme = ref('🎮 Gaming')
const markdown = ref('')
const currentFile = ref(null)
const zoom = ref(55)
const status = ref('READY')
const statusClass = ref('')
const cursorLine = ref(1)
const cursorCol = ref(1)
const files = ref([])
const batchProgress = ref({ active: false, current: 0, total: 0, currentFile: '' })
const pageCount = ref(0)
const renderedPages = ref([])
const isProcessing = ref(false)
const editorWidth = ref(450)

let updateTimeout = null
let isResizing = false

const startResize = (e) => {
  isResizing = true
  document.addEventListener('mousemove', onResize)
  document.addEventListener('mouseup', stopResize)
  document.body.style.cursor = 'col-resize'
  document.body.style.userSelect = 'none'
}

const onResize = (e) => {
  if (!isResizing) return
  const sidebar = 200
  const minWidth = 250
  const maxWidth = window.innerWidth - sidebar - 400
  editorWidth.value = Math.min(maxWidth, Math.max(minWidth, e.clientX - sidebar))
}

const stopResize = () => {
  isResizing = false
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
  document.body.style.cursor = ''
  document.body.style.userSelect = ''
}

const updatePreview = async () => {
  if (!markdown.value.trim()) {
    renderedPages.value = []
    pageCount.value = 0
    return
  }
  
  setStatus('RENDERING...', 'warning')
  isProcessing.value = true
  
  try {
    const css = getCSS(currentTheme.value)
    const html = marked(markdown.value)
    const pages = await splitIntoPages(html, css)
    pageCount.value = pages.length
    renderedPages.value = await generatePageHTMLArray(pages, css, currentTheme.value)
    setStatus(`${pages.length} PG`, 'success')
  } catch (e) {
    console.error(e)
    setStatus('ERROR', 'error')
  } finally {
    isProcessing.value = false
  }
}

watch([markdown, currentTheme], () => {
  clearTimeout(updateTimeout)
  updateTimeout = setTimeout(updatePreview, 500)
})

const loadFiles = async () => {
  try {
    setStatus('LOADING...', 'warning')
    const res = await fetch('/api/files')
    files.value = await res.json()
    setStatus(`${files.value.length} FILES`, 'success')
    if (files.value.length && !currentFile.value) loadFile(files.value[0])
  } catch (e) {
    setStatus('ERROR', 'error')
  }
}

const refreshFiles = () => loadFiles()
const loadFile = (file) => { currentFile.value = file.name; markdown.value = file.content }

const updateCursor = (e) => {
  const text = e.target.value.substring(0, e.target.selectionStart)
  const lines = text.split('\n')
  cursorLine.value = lines.length
  cursorCol.value = lines[lines.length - 1].length + 1
}

const setStatus = (text, type = '') => {
  status.value = text
  statusClass.value = type
  if (type === 'success') setTimeout(() => { status.value = 'READY'; statusClass.value = '' }, 2000)
}

const exportPDF = async () => {
  if (!markdown.value.trim() || isProcessing.value) return
  setStatus('EXPORTING...', 'warning')
  isProcessing.value = true
  
  try {
    const html2pdf = (await import('html2pdf.js')).default
    const css = getCSS(currentTheme.value)
    const html = marked(markdown.value)
    const pages = await splitIntoPages(html, css)
    const { container, styleEl } = await generatePDFPages(pages, css, currentTheme.value)
    
    document.body.appendChild(styleEl)
    document.body.appendChild(container)
    await new Promise(r => setTimeout(r, 500))
    
    await html2pdf(container, {
      margin: 0,
      filename: `${currentFile.value?.replace('.md', '').replace(/\//g, '_') || 'document'}.pdf`,
      image: { type: 'jpeg', quality: 0.95 },
      html2canvas: { 
        scale: 2, 
        useCORS: true, 
        logging: false, 
        windowWidth: 794,
        width: 794,
        height: 1123 * pages.length
      },
      jsPDF: { unit: 'mm', format: 'a4', orientation: 'portrait' },
      pagebreak: { mode: 'avoid-all', before: '.page' }
    })
    
    document.body.removeChild(container)
    document.body.removeChild(styleEl)
    setStatus('DONE!', 'success')
  } catch (e) {
    setStatus('ERROR', 'error')
    console.error(e)
  } finally {
    isProcessing.value = false
  }
}

const exportAllPDFs = async () => {
  if (files.value.length === 0 || isProcessing.value) return
  
  const html2pdf = (await import('html2pdf.js')).default
  const css = getCSS(currentTheme.value)
  
  batchProgress.value = { active: true, current: 0, total: files.value.length, currentFile: '' }
  isProcessing.value = true
  
  for (let i = 0; i < files.value.length; i++) {
    const file = files.value[i]
    batchProgress.value.current = i + 1
    batchProgress.value.currentFile = file.name
    
    try {
      const html = marked(file.content)
      const pages = await splitIntoPages(html, css)
      const { container, styleEl } = await generatePDFPages(pages, css, currentTheme.value)
      
      document.body.appendChild(styleEl)
      document.body.appendChild(container)
      await new Promise(r => setTimeout(r, 500))
      
      await html2pdf(container, {
        margin: 0,
        filename: `${file.name.replace('.md', '').replace(/\//g, '_')}.pdf`,
        image: { type: 'jpeg', quality: 0.95 },
        html2canvas: { 
          scale: 2, 
          useCORS: true, 
          logging: false, 
          windowWidth: 794,
          width: 794,
          height: 1123 * pages.length
        },
        jsPDF: { unit: 'mm', format: 'a4', orientation: 'portrait' },
        pagebreak: { mode: 'avoid-all', before: '.page' }
      })
      
      document.body.removeChild(container)
      document.body.removeChild(styleEl)
      await new Promise(r => setTimeout(r, 300))
    } catch (e) {
      console.error(`Error: ${file.name}`, e)
    }
  }
  
  batchProgress.value.active = false
  isProcessing.value = false
  setStatus(`${files.value.length} DONE!`, 'success')
}

onMounted(loadFiles)
onUnmounted(stopResize)
</script>

<style scoped>
.app { display: flex; flex-direction: column; height: 100vh; background: #050505; }

.header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 10px 16px;
  background: linear-gradient(180deg, #0f0f0f 0%, #0a0a0a 100%);
  border-bottom: 1px solid #1a1a1a;
}
.header-left, .header-center, .header-right { display: flex; align-items: center; gap: 10px; }

.logo { display: flex; align-items: center; gap: 6px; }
.logo-icon { font-size: 18px; filter: drop-shadow(0 0 6px rgba(34,197,94,0.5)); }
.logo-text { font-family: 'Orbitron', monospace; font-weight: 800; font-size: 13px; color: #22c55e; letter-spacing: 2px; }
.logo-version { font-size: 9px; color: #525252; padding: 2px 5px; background: #1a1a1a; border-radius: 3px; }

.btn { display: flex; align-items: center; gap: 5px; padding: 7px 12px; border: 1px solid #262626; border-radius: 6px; background: #141414; color: #e5e5e5; font-weight: 600; font-size: 10px; cursor: pointer; transition: all 0.15s; }
.btn:hover:not(:disabled) { border-color: #22c55e; }
.btn:disabled { opacity: 0.4; cursor: not-allowed; }
.btn-ghost { background: transparent; border-color: transparent; }
.btn-primary { background: #166534; border-color: #22c55e30; }
.btn-accent { background: #854d0e; border-color: #fbbf2430; }
.btn-icon { font-size: 11px; }

.status-badge { display: flex; align-items: center; gap: 5px; padding: 5px 10px; background: #141414; border: 1px solid #262626; border-radius: 12px; font-size: 9px; font-weight: 600; color: #525252; }
.status-dot { width: 5px; height: 5px; border-radius: 50%; background: #525252; }
.status-badge.success { color: #22c55e; }
.status-badge.success .status-dot { background: #22c55e; box-shadow: 0 0 6px #22c55e; }
.status-badge.warning { color: #fbbf24; }
.status-badge.warning .status-dot { background: #fbbf24; animation: pulse 1s infinite; }
.status-badge.error { color: #ef4444; }
.status-badge.error .status-dot { background: #ef4444; }

@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.4; } }

.main { display: flex; flex: 1; overflow: hidden; }

.sidebar { width: 200px; flex-shrink: 0; background: #0a0a0a; border-right: 1px solid #1a1a1a; display: flex; flex-direction: column; }
.sidebar-header { display: flex; align-items: center; justify-content: space-between; padding: 12px; border-bottom: 1px solid #1a1a1a; }
.sidebar-title { font-size: 9px; font-weight: 700; color: #22c55e; letter-spacing: 1px; }
.file-count { font-size: 9px; color: #525252; background: #1a1a1a; padding: 2px 6px; border-radius: 8px; }
.file-list { flex: 1; overflow-y: auto; padding: 6px; }
.file-item { display: flex; align-items: center; gap: 6px; padding: 8px 10px; border-radius: 6px; font-size: 10px; color: #737373; cursor: pointer; transition: all 0.1s; margin-bottom: 2px; }
.file-item:hover { background: #141414; color: #e5e5e5; }
.file-item.active { background: #166534; color: white; }
.file-icon { font-size: 10px; }
.file-name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

.editor-panel { display: flex; flex-direction: column; overflow: hidden; border-right: 1px solid #1a1a1a; min-width: 250px; }
.preview-panel { display: flex; flex-direction: column; overflow: hidden; flex: 1; min-width: 300px; }

.resizer {
  width: 6px; flex-shrink: 0;
  background: #1a1a1a; cursor: col-resize;
  transition: background 0.15s;
}
.resizer:hover { background: #22c55e; }

.panel-header { display: flex; align-items: center; justify-content: space-between; padding: 10px 14px; background: #0a0a0a; border-bottom: 1px solid #1a1a1a; flex-shrink: 0; }
.panel-title { display: flex; align-items: center; gap: 6px; font-size: 10px; font-weight: 700; color: #fbbf24; letter-spacing: 1px; }
.preview-header .panel-title { color: #22c55e; }
.panel-icon { font-size: 7px; }
.page-info { font-size: 9px; color: #525252; background: #1a1a1a; padding: 2px 6px; border-radius: 8px; margin-left: 6px; }
.panel-info { display: flex; gap: 10px; }
.cursor-pos { font-size: 9px; color: #525252; }

.editor { flex: 1; width: 100%; padding: 14px; background: #080808; border: none; color: #e5e5e5; font-family: 'JetBrains Mono', monospace; font-size: 11px; line-height: 1.6; resize: none; outline: none; }
.editor::placeholder { color: #333; }
.editor::selection { background: #166534; }

.preview-scroll {
  flex: 1; overflow: auto;
  background: radial-gradient(ellipse at center, #1a1a1a 0%, #0a0a0a 100%);
  padding: 24px;
}

.preview-pages {
  display: flex; flex-direction: column; align-items: center; gap: 24px;
  width: fit-content; margin: 0 auto;
}

.pdf-page {
  width: 210mm; height: 297mm;
  box-shadow: 0 8px 40px rgba(0,0,0,0.5), 0 0 0 1px rgba(255,255,255,0.04);
  border-radius: 2px; overflow: hidden; flex-shrink: 0;
}

.empty-preview {
  color: #525252; font-size: 14px; padding: 60px;
  display: flex; align-items: center; justify-content: center;
}

.zoom-control { display: flex; align-items: center; gap: 6px; }
.zoom-btn { width: 22px; height: 22px; border: 1px solid #262626; border-radius: 4px; background: #141414; color: #737373; font-size: 12px; cursor: pointer; }
.zoom-btn:hover { border-color: #22c55e; color: #22c55e; }
.zoom-value { font-size: 10px; color: #22c55e; min-width: 35px; text-align: center; }

.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.85); display: flex; align-items: center; justify-content: center; z-index: 1000; }
.modal { background: #141414; border: 1px solid #262626; border-radius: 12px; padding: 20px; min-width: 350px; }
.modal-header { font-size: 14px; font-weight: 700; color: #fbbf24; margin-bottom: 16px; }
.modal-body { display: flex; flex-direction: column; gap: 10px; }
.progress-info { font-size: 11px; color: #737373; }
.progress-file { font-size: 10px; color: #22c55e; font-family: monospace; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.progress-bar { height: 5px; background: #1a1a1a; border-radius: 3px; overflow: hidden; }
.progress-fill { height: 100%; background: linear-gradient(90deg, #22c55e, #4ade80); transition: width 0.2s; }
</style>
