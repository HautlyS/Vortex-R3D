"""Minimal Theme - Ultra Clean Modern Design"""

MINIMAL_CSS = '''
body {
  font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
  background: #fafafa;
  color: #171717;
}

.page {
  background: #ffffff;
  box-shadow: var(--shadow-lg);
}

h1 {
  color: #0a0a0a;
  letter-spacing: -0.03em;
  font-weight: 800;
  padding-bottom: var(--spacing-md);
  border-bottom: 2px solid #0a0a0a;
}

h2 {
  color: #171717;
  font-weight: 700;
  letter-spacing: -0.02em;
}

h3 {
  color: #262626;
  font-weight: 600;
}

h4 {
  color: #737373;
  font-weight: 500;
}

p { color: #404040; }
strong { color: #0a0a0a; }
em { color: #525252; }
a { color: #171717; text-decoration: underline; text-underline-offset: 2px; }
li::marker { color: #171717; }

/* Tables */
table {
  background: #ffffff;
  border: 1px solid #e5e5e5;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
}

thead {
  background: #fafafa;
}

th {
  color: #0a0a0a;
  border-bottom: 2px solid #171717;
  font-weight: 700;
}

td {
  color: #404040;
  border-bottom: 1px solid #f0f0f0;
}

tbody tr:nth-child(even) { background: #fafafa; }
tbody tr:last-child td { border-bottom: none; }

/* Code */
code {
  background: #f5f5f5;
  color: #171717;
  border: 1px solid #e5e5e5;
}

pre {
  background: #fafafa;
  color: #262626;
  border: 1px solid #e5e5e5;
}

blockquote {
  background: #fafafa;
  border-left: 3px solid #171717;
  color: #525252;
}

hr {
  background: #e5e5e5;
}

img {
  border: 1px solid #e5e5e5;
  box-shadow: var(--shadow-md);
}

@media print {
  body { background: #fff !important; }
}
'''
