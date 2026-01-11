"""Base CSS - Modern 2026 Professional A4 PDF Design System"""

BASE_CSS = '''
@import url('https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&family=DM+Sans:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&family=Orbitron:wght@500;600;700;800;900&family=Sora:wght@300;400;500;600;700&display=swap');

*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

@page { size: A4; margin: 0; }

:root {
  --spacing-xs: 4px;
  --spacing-sm: 8px;
  --spacing-md: 16px;
  --spacing-lg: 24px;
  --spacing-xl: 32px;
  --spacing-2xl: 48px;
  --radius-sm: 6px;
  --radius-md: 10px;
  --radius-lg: 14px;
  --shadow-sm: 0 1px 2px rgba(0,0,0,0.04);
  --shadow-md: 0 4px 12px rgba(0,0,0,0.06);
  --shadow-lg: 0 8px 24px rgba(0,0,0,0.08);
}

html, body {
  width: 210mm;
  min-height: 297mm;
  margin: 0 auto;
  padding: 0;
  -webkit-print-color-adjust: exact !important;
  print-color-adjust: exact !important;
}

body {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
  line-height: 1.7;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
  font-feature-settings: 'kern' 1, 'liga' 1;
}

.page {
  width: 210mm;
  min-height: 297mm;
  padding: 20mm 22mm 22mm 22mm;
  margin: 0 auto;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  align-items: center;
}

/* Subtle paper texture */
.page::before {
  content: '';
  position: absolute;
  inset: 0;
  pointer-events: none;
  opacity: 0.025;
  background-image: 
    url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
  z-index: 1;
}

/* Content wrapper - ensures centering */
.page > * {
  position: relative;
  z-index: 2;
  width: 100%;
  max-width: 166mm;
}

/* ============================================
   SMART PAGE BREAK RULES
   ============================================ */

table, thead, tbody, tr, pre, blockquote, figure, img, ul, ol {
  page-break-inside: avoid !important;
  break-inside: avoid !important;
}

h1, h2, h3, h4, h5, h6 {
  page-break-after: avoid !important;
  break-after: avoid !important;
  page-break-inside: avoid !important;
  break-inside: avoid !important;
}

p { orphans: 3; widows: 3; }

.page-break {
  page-break-before: always !important;
  break-before: page !important;
  height: 0;
  margin: 0;
  padding: 0;
}

li {
  page-break-inside: avoid !important;
  break-inside: avoid !important;
}

/* ============================================
   TYPOGRAPHY - CENTERED & MODERN
   ============================================ */

h1 {
  font-size: 22pt;
  font-weight: 800;
  margin: 0 0 var(--spacing-lg) 0;
  text-align: center;
  line-height: 1.25;
  letter-spacing: -0.02em;
  width: 100%;
}

h2 {
  font-size: 14pt;
  font-weight: 700;
  margin: var(--spacing-xl) 0 var(--spacing-md) 0;
  text-align: center;
  line-height: 1.35;
  letter-spacing: -0.01em;
  width: 100%;
}

h3 {
  font-size: 11.5pt;
  font-weight: 600;
  margin: var(--spacing-lg) 0 var(--spacing-sm) 0;
  text-align: center;
  line-height: 1.4;
  width: 100%;
}

h4 {
  font-size: 10pt;
  font-weight: 600;
  margin: var(--spacing-md) 0 var(--spacing-xs) 0;
  text-align: center;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  width: 100%;
}

p {
  font-size: 9.5pt;
  margin: var(--spacing-sm) 0;
  text-align: justify;
  hyphens: auto;
  line-height: 1.75;
}

strong { font-weight: 700; }
em { font-style: italic; }

/* Lists - Centered container */
ul, ol {
  margin: var(--spacing-md) auto;
  padding-left: 24px;
  max-width: 140mm;
}

li {
  margin: var(--spacing-xs) 0;
  font-size: 9.5pt;
  line-height: 1.65;
}

li::marker { font-weight: 600; }

/* ============================================
   TABLES - MODERN CENTERED DESIGN
   ============================================ */

table {
  width: 100%;
  max-width: 100%;
  border-collapse: separate;
  border-spacing: 0;
  margin: var(--spacing-lg) auto;
  font-size: 8.5pt;
  border-radius: var(--radius-md);
  overflow: hidden;
}

thead {
  text-align: center;
}

th, td {
  padding: 12px 14px;
  text-align: center;
  vertical-align: middle;
}

th {
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  font-size: 7.5pt;
}

td {
  font-size: 8.5pt;
  line-height: 1.5;
}

/* First column left-aligned for readability */
th:first-child,
td:first-child {
  text-align: left;
  padding-left: 16px;
}

/* Last column right-aligned for numbers */
th:last-child,
td:last-child {
  text-align: right;
  padding-right: 16px;
}

/* Single column tables stay centered */
table:has(th:only-child) th,
table:has(td:only-child) td {
  text-align: center;
}

/* ============================================
   CODE BLOCKS
   ============================================ */

code {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 8pt;
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  font-feature-settings: 'liga' 0;
}

pre {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 7.5pt;
  padding: var(--spacing-md);
  margin: var(--spacing-md) auto;
  border-radius: var(--radius-md);
  overflow-x: auto;
  line-height: 1.6;
  max-width: 100%;
}

pre code {
  padding: 0;
  background: transparent;
  border: none;
}

/* ============================================
   BLOCKQUOTES - CENTERED
   ============================================ */

blockquote {
  margin: var(--spacing-lg) auto;
  padding: var(--spacing-md) var(--spacing-lg);
  border-radius: 0 var(--radius-md) var(--radius-md) 0;
  font-style: italic;
  max-width: 150mm;
  text-align: center;
}

blockquote p {
  margin: 0;
  font-size: 10pt;
  text-align: center;
}

/* ============================================
   OTHER ELEMENTS
   ============================================ */

hr {
  border: none;
  height: 2px;
  margin: var(--spacing-xl) auto;
  max-width: 80mm;
  border-radius: 1px;
}

img {
  max-width: 100%;
  height: auto;
  display: block;
  margin: var(--spacing-lg) auto;
  border-radius: var(--radius-md);
}

a {
  text-decoration: none;
  font-weight: 500;
  transition: opacity 0.2s ease;
}

/* ============================================
   PRINT STYLES
   ============================================ */

@media print {
  html, body {
    width: 210mm !important;
    margin: 0 !important;
  }
  .page {
    margin: 0 !important;
    box-shadow: none !important;
    page-break-after: always;
  }
}
'''
