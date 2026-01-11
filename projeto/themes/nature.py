"""Nature Theme - Fresh Organic Forest Green"""

NATURE_CSS = '''
body {
  font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
  background: #e8f0eb;
  color: #2d3b2d;
}

.page {
  background: linear-gradient(180deg, #f7faf8 0%, #f2f7f4 100%);
  box-shadow: var(--shadow-lg);
}

.page::after {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0;
  height: 3px;
  background: linear-gradient(90deg, #16a34a 0%, #22c55e 50%, #16a34a 100%);
  z-index: 10;
}

h1 {
  color: #166534;
  font-weight: 800;
  letter-spacing: -0.01em;
}

h2 {
  color: #15803d;
  font-weight: 700;
  padding: var(--spacing-sm) var(--spacing-md);
  background: linear-gradient(90deg, rgba(22,163,74,0.08) 0%, transparent 70%);
  border-left: 3px solid #22c55e;
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  text-align: left;
}

h3 { color: #166534; }

h4 {
  color: #4ade80;
  font-weight: 500;
}

p { color: #3d4d3d; }
strong { color: #166534; }
em { color: #15803d; }
a { color: #16a34a; }
li::marker { color: #22c55e; }

/* Tables */
table {
  background: #ffffff;
  border: 1px solid rgba(22,163,74,0.15);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
}

thead {
  background: linear-gradient(180deg, rgba(22,163,74,0.1) 0%, rgba(22,163,74,0.04) 100%);
}

th {
  color: #166534;
  border-bottom: 2px solid rgba(22,163,74,0.25);
  font-weight: 700;
}

td {
  color: #3d4d3d;
  border-bottom: 1px solid rgba(22,163,74,0.08);
}

tbody tr:nth-child(even) { background: rgba(22,163,74,0.03); }
tbody tr:last-child td { border-bottom: none; }

/* Code */
code {
  background: rgba(22,163,74,0.1);
  color: #166534;
  border: 1px solid rgba(22,163,74,0.15);
}

pre {
  background: #f7faf8;
  color: #2d5a2d;
  border: 1px solid rgba(22,163,74,0.15);
}

blockquote {
  background: rgba(22,163,74,0.06);
  border-left: 3px solid #22c55e;
  color: #4a7c4a;
}

hr {
  background: linear-gradient(90deg, transparent, #22c55e, transparent);
}

img {
  border: 1px solid rgba(22,163,74,0.12);
  box-shadow: var(--shadow-md);
}

@media print {
  body { background: #f2f7f4 !important; }
}
'''
