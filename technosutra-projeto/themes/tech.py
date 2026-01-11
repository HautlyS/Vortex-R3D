"""Tech Theme - Clean Futuristic Blue with Subtle Glow"""

TECH_CSS = '''
body {
  font-family: 'Sora', system-ui, sans-serif;
  background: #070b14;
  color: #e4eaf2;
}

.page {
  background: linear-gradient(180deg, #0d1320 0%, #080c16 100%);
  border: 1px solid rgba(56,189,248,0.08);
  box-shadow: 
    0 0 60px rgba(14,165,233,0.04),
    inset 0 1px 0 rgba(56,189,248,0.05);
}

.page::after {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent 5%, #0ea5e9 30%, #38bdf8 50%, #0ea5e9 70%, transparent 95%);
  z-index: 10;
}

h1 {
  color: #38bdf8;
  letter-spacing: 0.04em;
  text-shadow: 0 0 40px rgba(56,189,248,0.3);
}

h2 {
  color: #7dd3fc;
  padding: var(--spacing-sm) var(--spacing-md);
  background: linear-gradient(90deg, rgba(14,165,233,0.08) 0%, transparent 70%);
  border-left: 3px solid #0ea5e9;
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  text-align: left;
}

h3 { color: #bae6fd; }

h4 {
  color: #0ea5e9;
  font-weight: 500;
}

p { color: #b8c9db; }
strong { color: #38bdf8; }
em { color: #7dd3fc; font-style: normal; }
a { color: #0ea5e9; }
li::marker { color: #0ea5e9; }

/* Tables */
table {
  background: rgba(13,19,32,0.9);
  border: 1px solid rgba(14,165,233,0.15);
  border-radius: var(--radius-md);
  box-shadow: 
    0 4px 24px rgba(0,0,0,0.3),
    0 0 20px rgba(14,165,233,0.05);
}

thead {
  background: linear-gradient(180deg, rgba(14,165,233,0.12) 0%, rgba(14,165,233,0.06) 100%);
}

th {
  color: #38bdf8;
  border-bottom: 1px solid rgba(14,165,233,0.3);
  text-shadow: 0 0 10px rgba(56,189,248,0.2);
}

td {
  color: #d4e4f2;
  border-bottom: 1px solid rgba(14,165,233,0.08);
}

tbody tr:nth-child(even) { background: rgba(14,165,233,0.03); }
tbody tr:last-child td { border-bottom: none; }

/* Code */
code {
  background: rgba(14,165,233,0.12);
  color: #7dd3fc;
  border: 1px solid rgba(14,165,233,0.2);
}

pre {
  background: rgba(8,12,22,0.95);
  color: #7dd3fc;
  border: 1px solid rgba(14,165,233,0.15);
  box-shadow: inset 0 1px 3px rgba(0,0,0,0.2);
}

blockquote {
  background: rgba(14,165,233,0.06);
  border-left: 3px solid #0ea5e9;
  color: #bae6fd;
}

hr {
  background: linear-gradient(90deg, transparent, #0ea5e9, transparent);
  box-shadow: 0 0 8px rgba(14,165,233,0.3);
}

img {
  border: 1px solid rgba(14,165,233,0.15);
  box-shadow: 0 4px 20px rgba(0,0,0,0.4);
}

@media print {
  body { background: #080c16 !important; }
}
'''
