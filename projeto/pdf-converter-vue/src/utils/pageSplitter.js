// Smart A4 Page Splitter - Section-aware with SP Branding
// Keeps headers with their content blocks

const A4_HEIGHT_MM = 297
const MARGIN_TOP_MM = 18
const MARGIN_BOTTOM_MM = 18
const CONTENT_HEIGHT_MM = A4_HEIGHT_MM - MARGIN_TOP_MM - MARGIN_BOTTOM_MM
const MM_TO_PX = 3.7795275591
export const CONTENT_HEIGHT_PX = CONTENT_HEIGHT_MM * MM_TO_PX

const SECTION_HEADERS = ['H1', 'H2', 'H3', 'H4', 'H5', 'H6']
const UNSPLITTABLE = ['TABLE', 'THEAD', 'TBODY', 'TR', 'PRE', 'BLOCKQUOTE', 'UL', 'OL', 'FIGURE', 'IMG', 'DL']

// Get element height including margins
const getHeight = (el) => {
  const rect = el.getBoundingClientRect()
  const style = getComputedStyle(el)
  return rect.height + (parseFloat(style.marginTop) || 0) + (parseFloat(style.marginBottom) || 0)
}

// Calculate how much content follows a header (until next header of same/higher level)
const getFollowingContentHeight = (children, startIdx, maxHeight) => {
  const header = children[startIdx]
  const headerLevel = parseInt(header.tagName[1])
  let height = 0
  
  for (let i = startIdx + 1; i < children.length && height < maxHeight; i++) {
    const child = children[i]
    // Stop at next header of same or higher level
    if (SECTION_HEADERS.includes(child.tagName) && parseInt(child.tagName[1]) <= headerLevel) break
    height += getHeight(child)
  }
  return Math.min(height, maxHeight)
}

export function splitIntoPages(htmlContent, css) {
  return new Promise((resolve) => {
    const container = document.createElement('div')
    container.style.cssText = 'position:absolute;left:-9999px;top:0;width:210mm;visibility:hidden;'
    container.innerHTML = `<style>${css}</style><div class="page" style="padding:${MARGIN_TOP_MM}mm 20mm ${MARGIN_BOTTOM_MM}mm 20mm;">${htmlContent}</div>`
    document.body.appendChild(container)

    requestAnimationFrame(() => {
      setTimeout(() => {
        const pageDiv = container.querySelector('.page')
        const children = Array.from(pageDiv.children).filter(c => c.tagName !== 'STYLE')
        
        const pages = []
        let currentElements = []
        let currentHeight = 0
        
        // Minimum content to keep with header (at least 1 paragraph or ~150px)
        const MIN_CONTENT_WITH_HEADER = 150
        
        for (let i = 0; i < children.length; i++) {
          const child = children[i]
          const childHeight = getHeight(child)
          const isHeader = SECTION_HEADERS.includes(child.tagName)
          const isUnsplittable = UNSPLITTABLE.includes(child.tagName) || child.querySelector(UNSPLITTABLE.join(','))
          
          // For headers: calculate space needed for header + following content
          let requiredHeight = childHeight
          if (isHeader) {
            const followingHeight = getFollowingContentHeight(children, i, MIN_CONTENT_WITH_HEADER)
            requiredHeight = childHeight + followingHeight
          }
          
          const wouldExceed = currentHeight + childHeight > CONTENT_HEIGHT_PX
          const headerWouldBeOrphan = isHeader && (currentHeight + requiredHeight > CONTENT_HEIGHT_PX)
          const unsplittableWouldExceed = isUnsplittable && wouldExceed
          
          // Start new page if header would be orphaned or content exceeds
          if ((headerWouldBeOrphan || (wouldExceed && !isHeader) || unsplittableWouldExceed) && currentElements.length > 0) {
            pages.push(currentElements.map(el => el.outerHTML).join('\n'))
            currentElements = []
            currentHeight = 0
          }
          
          currentElements.push(child.cloneNode(true))
          currentHeight += childHeight
          
          // Handle oversized single elements
          if (childHeight > CONTENT_HEIGHT_PX && currentElements.length === 1) {
            pages.push(currentElements.map(el => el.outerHTML).join('\n'))
            currentElements = []
            currentHeight = 0
          }
        }
        
        if (currentElements.length > 0) {
          pages.push(currentElements.map(el => el.outerHTML).join('\n'))
        }
        
        document.body.removeChild(container)
        resolve(pages.length > 0 ? pages : [htmlContent])
      }, 150)
    })
  })
}

// Theme detection
const DARK_THEMES = ['🎮 Gaming', '💜 Neon', '👑 Luxury', '🔷 Tech']
export const isThemeDark = (theme) => DARK_THEMES.includes(theme)

// Convert images to base64 for reliable embedding
let cachedImages = null
async function loadImages() {
  if (cachedImages) return cachedImages
  
  const toBase64 = async (url) => {
    try {
      const res = await fetch(url)
      const blob = await res.blob()
      return new Promise((resolve) => {
        const reader = new FileReader()
        reader.onloadend = () => resolve(reader.result)
        reader.readAsDataURL(blob)
      })
    } catch (e) {
      console.warn('Failed to load image:', url)
      return ''
    }
  }
  
  cachedImages = {
    faixa: await toBase64('/assets/sp-faixa.png'),
    logo: await toBase64('/assets/logo-sp.png'),
    bandeira: await toBase64('/assets/bandeira-sp.png')
  }
  return cachedImages
}

// Generate layout CSS
const getLayoutCSS = (isDark, images) => `
/* Header - Repeating banner */
.sp-header-bg {
  position: absolute;
  top: 0; left: 0; right: 0;
  height: 16mm;
  background: url("${images.faixa}") repeat-x left center;
  background-size: auto 14mm;
  opacity: ${isDark ? '0.08' : '0.12'};
  z-index: 50;
  pointer-events: none;
}

/* Footer - Clean minimal design */
.sp-footer {
  position: absolute;
  bottom: 5mm; left: 8mm; right: 8mm;
  height: 8mm;
  display: flex;
  align-items: center;
  justify-content: space-between;
  z-index: 100;
  font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
  border-top: 0.5px solid ${isDark ? 'rgba(255,255,255,0.08)' : 'rgba(0,0,0,0.06)'};
  padding-top: 2mm;
}

.sp-footer-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

/* Logo filter: for dark themes, invert black while trying to preserve red tones */
.sp-logo {
  height: 6mm;
  width: auto;
  opacity: ${isDark ? '0.85' : '0.6'};
  ${isDark ? `
  /* Invert + hue-rotate to bring red back after inversion */
  filter: invert(1) hue-rotate(180deg) saturate(1.5) brightness(1.1);
  ` : ''}
}

/* Red heart preserved via separate element if needed */
.sp-logo-heart {
  position: absolute;
  height: 6mm;
  opacity: 0.8;
}

.sp-footer-center {
  flex: 1;
  text-align: center;
}

.sp-edital {
  font-size: 5.5pt;
  font-weight: 500;
  letter-spacing: 0.5px;
  color: ${isDark ? 'rgba(255,255,255,0.3)' : 'rgba(0,0,0,0.3)'};
}

.sp-footer-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.sp-bandeira {
  height: 4.5mm;
  width: auto;
  border-radius: 1px;
  opacity: ${isDark ? '0.5' : '0.45'};
  box-shadow: 0 0 2px rgba(0,0,0,0.1);
}

.sp-page {
  font-size: 6.5pt;
  font-weight: 600;
  color: ${isDark ? 'rgba(255,255,255,0.4)' : 'rgba(0,0,0,0.35)'};
  font-family: 'JetBrains Mono', monospace;
  letter-spacing: 0.5px;
}

/* Content */
.page-content {
  position: relative;
  z-index: 10;
}

/* Ensure backgrounds cover full page */
.page::before, .page::after {
  position: absolute !important;
  top: 0 !important; left: 0 !important;
  right: 0 !important; bottom: 0 !important;
}
`

// Generate page HTML
const generatePageHTML = (content, pageNum, totalPages, isDark, images) => `
<div class="page">
  <div class="sp-header-bg"></div>
  <div class="page-content">${content}</div>
  <div class="sp-footer">
    <div class="sp-footer-left">
      <img src="${images.logo}" alt="SP" class="sp-logo" />
    </div>
    <div class="sp-footer-center">
      <span class="sp-edital">FOMENTO CULTSP - PNAB Nº 12/2025</span>
    </div>
    <div class="sp-footer-right">
      <img src="${images.bandeira}" alt="" class="sp-bandeira" />
      <span class="sp-page">${pageNum} / ${totalPages}</span>
    </div>
  </div>
</div>`

// Multi-page preview - returns array of HTML strings for direct rendering
export async function generatePageHTMLArray(pages, css, themeName) {
  const isDark = isThemeDark(themeName)
  const images = await loadImages()
  const layoutCSS = getLayoutCSS(isDark, images)
  
  const baseStyle = `
    <style>
    ${css}
    ${layoutCSS}
    .page {
      width: 210mm; height: 297mm;
      position: relative;
      padding: ${MARGIN_TOP_MM}mm 20mm ${MARGIN_BOTTOM_MM}mm 20mm;
      box-sizing: border-box;
      overflow: hidden;
    }
    </style>
  `
  
  return pages.map((content, i) => 
    baseStyle + generatePageHTML(content, i + 1, pages.length, isDark, images)
  )
}

// Legacy multi-page HTML (for iframe if needed)
export async function generateMultiPageHTML(pages, css, themeName) {
  const isDark = isThemeDark(themeName)
  const images = await loadImages()
  const layoutCSS = getLayoutCSS(isDark, images)
  
  const pagesHTML = pages.map((content, i) => 
    generatePageHTML(content, i + 1, pages.length, isDark, images)
  ).join('\n<div class="page-gap"></div>\n')
  
  return `<!DOCTYPE html>
<html lang="pt-BR">
<head>
<meta charset="UTF-8">
<style>
${css}
${layoutCSS}

html, body {
  background: #1a1a1a !important;
  margin: 0; padding: 25px;
}

.page {
  width: 210mm; height: 297mm;
  min-height: 297mm; max-height: 297mm;
  overflow: hidden;
  margin: 0 auto;
  box-shadow: 0 8px 40px rgba(0,0,0,0.5), 0 0 0 1px rgba(255,255,255,0.04);
  border-radius: 2px;
  position: relative;
  padding: ${MARGIN_TOP_MM}mm 20mm ${MARGIN_BOTTOM_MM}mm 20mm;
  box-sizing: border-box;
}

.page-gap { height: 30px; }
</style>
</head>
<body>${pagesHTML}</body>
</html>`
}

// PDF export - returns DOM elements for html2pdf
export async function generatePDFPages(pages, css, themeName) {
  const isDark = isThemeDark(themeName)
  const images = await loadImages()
  const layoutCSS = getLayoutCSS(isDark, images)
  
  // Create style element with all necessary styles
  const styleEl = document.createElement('style')
  styleEl.textContent = `
    ${css}
    ${layoutCSS}
    
    .pdf-container { 
      width: 210mm; 
      margin: 0; 
      padding: 0;
    }
    
    .page {
      width: 210mm; 
      height: 297mm;
      min-height: 297mm; 
      max-height: 297mm;
      overflow: hidden;
      position: relative;
      padding: ${MARGIN_TOP_MM}mm 20mm ${MARGIN_BOTTOM_MM}mm 20mm;
      box-sizing: border-box;
      page-break-after: always;
      page-break-inside: avoid;
      break-after: page;
      break-inside: avoid;
    }
    
    .page:last-child {
      page-break-after: avoid;
      break-after: avoid;
    }
  `
  
  // Create container with pages
  const container = document.createElement('div')
  container.className = 'pdf-container'
  
  pages.forEach((content, i) => {
    const pageEl = document.createElement('div')
    pageEl.className = 'page'
    pageEl.innerHTML = `
      <div class="sp-header-bg"></div>
      <div class="page-content">${content}</div>
      <div class="sp-footer">
        <div class="sp-footer-left">
          <img src="${images.logo}" alt="SP" class="sp-logo" />
        </div>
        <div class="sp-footer-center">
          <span class="sp-edital">FOMENTO CULTSP - PNAB Nº 12/2025</span>
        </div>
        <div class="sp-footer-right">
          <img src="${images.bandeira}" alt="" class="sp-bandeira" />
          <span class="sp-page">${i + 1} / ${pages.length}</span>
        </div>
      </div>
    `
    container.appendChild(pageEl)
  })
  
  return { container, styleEl }
}
