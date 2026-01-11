"""Corporate Theme - Professional Trust-Building Blue"""

CORPORATE_CSS = '''
body {
  font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
  background: #f1f5f9;
  color: #1e293b;
}

.page {
  background: #ffffff;
  box-shadow: var(--shadow-lg);
}

.page::after {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0;
  height: 3px;
  background: linear-gradient(90deg, #1e40af 0%, #3b82f6 50%, #1e40af 100%);
  z-index: 10;
}

h1 {
  color: #1e40af;
  letter-spacing: -0.02em;
  font-weight: 800;
}

h2 {
  color: #1e293b;
  font-weight: 700;
  padding-bottom: var(--spacing-sm);
  border-bottom: 1px solid #e2e8f0;
}

h3 { color: #334155; }

h4 {
  color: #64748b;
  font-weight: 500;
}

p { color: #475569; }
strong { color: #1e293b; }
em { color: #1e40af; }
a { color: #2563eb; }
li::marker { color: #3b82f6; }

/* Tables */
table {
  background: #ffffff;
  border: 1px solid #e2e8f0;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
}

thead {
  background: linear-gradient(180deg, #f8fafc 0%, #f1f5f9 100%);
}

th {
  color: #1e40af;
  border-bottom: 2px solid #e2e8f0;
  font-weight: 700;
}

td {
  color: #475569;
  border-bottom: 1px solid #f1f5f9;
}

tbody tr:nth-child(even) { background: #f8fafc; }
tbody tr:last-child td { border-bottom: none; }

/* Code */
code {
  background: #f1f5f9;
  color: #1e40af;
  border: 1px solid #e2e8f0;
}

pre {
  background: #f8fafc;
  color: #334155;
  border: 1px solid #e2e8f0;
}

blockquote {
  background: #eff6ff;
  border-left: 3px solid #3b82f6;
  color: #475569;
}

hr {
  background: #e2e8f0;
}

img {
  border: 1px solid #e2e8f0;
  box-shadow: var(--shadow-md);
}

@media print {
  body { background: #fff !important; }
}
'''
