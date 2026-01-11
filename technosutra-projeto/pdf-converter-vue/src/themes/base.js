// Base CSS for A4 PDF - Professional Layout with proper alignment
export const BASE_CSS = `
@import url('https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&family=Orbitron:wght@500;600;700;800;900&family=Sora:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap');

*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
@page { size: A4; margin: 0; }

:root {
  --spacing-xs: 4px; --spacing-sm: 8px; --spacing-md: 16px;
  --spacing-lg: 24px; --spacing-xl: 32px; --spacing-2xl: 48px;
  --radius-sm: 6px; --radius-md: 10px; --radius-lg: 14px;
}

.page {
  width: 210mm;
  min-height: 297mm;
  margin: 0 auto;
  position: relative;
  line-height: 1.7;
  -webkit-print-color-adjust: exact !important;
  print-color-adjust: exact !important;
}

/* Paper texture overlay */
.page::before {
  content: '';
  position: absolute;
  inset: 0;
  pointer-events: none;
  opacity: 0.025;
  background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
  z-index: 1;
}

/* Mandala watermark */
.page::after {
  content: '';
  position: absolute;
  bottom: 25mm; right: 20mm;
  width: 50mm; height: 50mm;
  pointer-events: none;
  opacity: 0.03;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'%3E%3Cg fill='none' stroke='currentColor' stroke-width='0.5'%3E%3Ccircle cx='100' cy='100' r='95'/%3E%3Ccircle cx='100' cy='100' r='75'/%3E%3Ccircle cx='100' cy='100' r='55'/%3E%3Ccircle cx='100' cy='100' r='35'/%3E%3Cpath d='M100 5v190M5 100h190'/%3E%3Cpath d='M30 30L170 170M170 30L30 170'/%3E%3C/g%3E%3C/svg%3E");
  background-size: contain;
  background-repeat: no-repeat;
  z-index: 1;
}

.page > *, .page-content > * { position: relative; z-index: 2; }

/* Page break rules - NEVER break components */
table, thead, tbody, tr, pre, blockquote, figure, img, ul, ol, li, dl, dt, dd {
  page-break-inside: avoid !important;
  break-inside: avoid !important;
}
h1, h2, h3, h4, h5, h6 {
  page-break-after: avoid !important;
  break-after: avoid !important;
  page-break-inside: avoid !important;
}
p { orphans: 3; widows: 3; }

/* ===========================================
   TYPOGRAPHY - Professional Alignment
   =========================================== */

/* H1 - Centered title */
h1 {
  font-size: 20pt;
  font-weight: 800;
  margin: 0 0 var(--spacing-lg) 0;
  text-align: center;
  line-height: 1.3;
  letter-spacing: -0.01em;
}

/* H2-H6 - Left aligned section headers */
h2 {
  font-size: 13pt;
  font-weight: 700;
  margin: var(--spacing-xl) 0 var(--spacing-md) 0;
  text-align: left;
  line-height: 1.35;
}

h3 {
  font-size: 11pt;
  font-weight: 600;
  margin: var(--spacing-lg) 0 var(--spacing-sm) 0;
  text-align: left;
}

h4 {
  font-size: 10pt;
  font-weight: 600;
  margin: var(--spacing-md) 0 var(--spacing-xs) 0;
  text-align: left;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

h5, h6 {
  font-size: 9pt;
  font-weight: 600;
  margin: var(--spacing-sm) 0;
  text-align: left;
}

/* Paragraphs - Justified */
p {
  font-size: 9.5pt;
  margin: var(--spacing-sm) 0;
  text-align: justify;
  text-justify: inter-word;
  hyphens: auto;
  -webkit-hyphens: auto;
  line-height: 1.75;
}

strong { font-weight: 700; }
em { font-style: italic; }

/* Lists - Left aligned */
ul, ol {
  margin: var(--spacing-md) 0;
  padding-left: 2em;
  text-align: left;
}

li {
  margin: var(--spacing-xs) 0;
  font-size: 9.5pt;
  line-height: 1.65;
  text-align: left;
}

li::marker { font-weight: 600; }

/* Nested lists */
li ul, li ol {
  margin: var(--spacing-xs) 0;
}

/* ===========================================
   TABLES - Centered with proper alignment
   =========================================== */

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

thead { text-align: center; }

th {
  padding: 10px 12px;
  text-align: center;
  vertical-align: middle;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-size: 7.5pt;
}

td {
  padding: 10px 12px;
  text-align: left;
  vertical-align: middle;
  font-size: 8.5pt;
  line-height: 1.5;
}

/* First column - left aligned (labels) */
td:first-child {
  text-align: left;
  font-weight: 500;
}

/* Last column - right aligned (values/numbers) */
td:last-child:not(:first-child) {
  text-align: right;
}

/* Single column tables stay left */
table:has(td:only-child) td {
  text-align: left;
}

/* ===========================================
   CODE BLOCKS - Left aligned
   =========================================== */

code {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 8pt;
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  text-align: left;
}

pre {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 7.5pt;
  padding: var(--spacing-md);
  margin: var(--spacing-md) 0;
  border-radius: var(--radius-md);
  overflow-x: auto;
  line-height: 1.6;
  text-align: left;
  white-space: pre-wrap;
  word-wrap: break-word;
}

pre code {
  padding: 0;
  background: transparent;
  border: none;
}

/* ===========================================
   BLOCKQUOTES - Left aligned with indent
   =========================================== */

blockquote {
  margin: var(--spacing-lg) 2em;
  padding: var(--spacing-md) var(--spacing-lg);
  border-radius: 0 var(--radius-md) var(--radius-md) 0;
  text-align: left;
}

blockquote p {
  margin: 0;
  font-size: 9.5pt;
  text-align: left;
  font-style: italic;
}

/* ===========================================
   OTHER ELEMENTS
   =========================================== */

hr {
  border: none;
  height: 1px;
  margin: var(--spacing-xl) auto;
  max-width: 60%;
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
}

/* Definition lists */
dl {
  margin: var(--spacing-md) 0;
}

dt {
  font-weight: 700;
  margin-top: var(--spacing-sm);
}

dd {
  margin-left: 2em;
  margin-bottom: var(--spacing-xs);
}

/* ===========================================
   PRINT STYLES
   =========================================== */

@media print {
  .page {
    margin: 0 !important;
    box-shadow: none !important;
    page-break-after: always;
  }
}
`
