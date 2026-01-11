"""Zen Theme - Peaceful Organic Minimalism"""

ZEN_CSS = '''
@import url('https://fonts.googleapis.com/css2?family=Crimson+Pro:wght@400;500;600&display=swap');

body {
  font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
  background: #f5f3ef;
  color: #3d3d3d;
}

.page {
  background: linear-gradient(180deg, #faf9f7 0%, #f5f3ef 100%);
  box-shadow: var(--shadow-md);
}

h1 {
  font-family: 'Crimson Pro', Georgia, serif;
  font-weight: 500;
  color: #5c4a3d;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

h2 {
  font-family: 'Crimson Pro', Georgia, serif;
  font-weight: 500;
  color: #6b5b4f;
  letter-spacing: 0.02em;
  padding-bottom: var(--spacing-sm);
  border-bottom: 1px solid rgba(139,119,101,0.25);
}

h3 { color: #7a6a5e; }

h4 {
  color: #9a8a7e;
  font-weight: 500;
}

p { color: #4a4a4a; }
strong { color: #5c4a3d; }
em { color: #8b7765; }
a { color: #8b7765; }
li::marker { color: #a89888; }

/* Tables */
table {
  background: #faf9f7;
  border: 1px solid rgba(139,119,101,0.15);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
}

thead {
  background: rgba(139,119,101,0.06);
}

th {
  color: #5c4a3d;
  border-bottom: 1px solid rgba(139,119,101,0.2);
  font-weight: 600;
}

td {
  color: #4a4a4a;
  border-bottom: 1px solid rgba(139,119,101,0.08);
}

tbody tr:nth-child(even) { background: rgba(139,119,101,0.03); }
tbody tr:last-child td { border-bottom: none; }

/* Code */
code {
  background: rgba(139,119,101,0.1);
  color: #5c4a3d;
  border: none;
}

pre {
  background: rgba(139,119,101,0.06);
  color: #4a4a4a;
  border: 1px solid rgba(139,119,101,0.1);
}

blockquote {
  background: rgba(139,119,101,0.05);
  border-left: 2px solid #a89888;
  color: #6b5b4f;
}

hr {
  background: rgba(139,119,101,0.2);
}

img {
  border: 1px solid rgba(139,119,101,0.1);
  box-shadow: var(--shadow-sm);
}

@media print {
  body { background: #f5f3ef !important; }
}
'''
