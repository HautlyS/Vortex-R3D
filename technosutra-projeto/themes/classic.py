"""Classic Theme - Refined Elegant Serif with Warm Tones"""

CLASSIC_CSS = '''
body {
  font-family: 'DM Sans', Georgia, serif;
  background: #f7f4ef;
  color: #2a2218;
}

.page {
  background: linear-gradient(180deg, #fdfcfa 0%, #f9f7f3 100%);
  box-shadow: var(--shadow-lg);
}

h1 {
  color: #6b4423;
  letter-spacing: 0.01em;
  padding-bottom: var(--spacing-md);
  border-bottom: 3px solid #c9a86c;
  background: linear-gradient(180deg, #7a5230 0%, #5c3d1e 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

h2 {
  color: #6b4423;
  padding: var(--spacing-sm) 0;
  border-bottom: 1px solid rgba(201,168,108,0.4);
}

h3 { color: #7a5230; }

h4 {
  color: #9a7b5a;
  font-weight: 500;
}

p { color: #3d3020; }
strong { color: #5c3d1e; }
em { color: #8b6340; }
a { color: #8b4513; }
li::marker { color: #c9a86c; }

/* Tables */
table {
  background: #fdfcfa;
  border: 1px solid rgba(201,168,108,0.3);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
}

thead {
  background: linear-gradient(180deg, rgba(201,168,108,0.12) 0%, rgba(201,168,108,0.06) 100%);
}

th {
  color: #5c3d1e;
  border-bottom: 2px solid rgba(201,168,108,0.5);
  font-weight: 700;
}

td {
  color: #3d3020;
  border-bottom: 1px solid rgba(201,168,108,0.15);
}

tbody tr:nth-child(even) { background: rgba(201,168,108,0.04); }
tbody tr:last-child td { border-bottom: none; }

/* Code */
code {
  background: rgba(201,168,108,0.12);
  color: #5c3d1e;
  border: 1px solid rgba(201,168,108,0.2);
}

pre {
  background: linear-gradient(180deg, #f9f7f3 0%, #f5f2ed 100%);
  color: #3d3020;
  border: 1px solid rgba(201,168,108,0.2);
  box-shadow: inset 0 1px 3px rgba(0,0,0,0.03);
}

blockquote {
  background: rgba(201,168,108,0.08);
  border-left: 4px solid #c9a86c;
  color: #6b4423;
}

hr {
  background: linear-gradient(90deg, transparent, #c9a86c, transparent);
}

img {
  border: 1px solid rgba(201,168,108,0.2);
  box-shadow: var(--shadow-md);
}

@media print {
  body { background: #f9f7f3 !important; }
}
'''
