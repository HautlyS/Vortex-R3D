import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { readdirSync, readFileSync, statSync } from 'fs'
import { join, relative } from 'path'

// Plugin to serve markdown files from projeto/
function markdownFilesPlugin() {
  const projetoPath = join(__dirname, '..')
  
  function getMarkdownFiles(dir, files = []) {
    const items = readdirSync(dir)
    for (const item of items) {
      if (item === 'node_modules' || item === 'pdf-converter-vue' || item.startsWith('.')) continue
      const fullPath = join(dir, item)
      const stat = statSync(fullPath)
      if (stat.isDirectory()) {
        getMarkdownFiles(fullPath, files)
      } else if (item.endsWith('.md')) {
        files.push({
          path: fullPath,
          name: relative(projetoPath, fullPath),
          content: readFileSync(fullPath, 'utf-8')
        })
      }
    }
    return files
  }

  return {
    name: 'markdown-files',
    configureServer(server) {
      server.middlewares.use('/api/files', (req, res) => {
        const files = getMarkdownFiles(projetoPath)
        res.setHeader('Content-Type', 'application/json')
        res.end(JSON.stringify(files))
      })
    }
  }
}

export default defineConfig({
  plugins: [vue(), markdownFilesPlugin()],
  server: { port: 3000, open: true }
})
