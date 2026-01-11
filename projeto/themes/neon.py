"""Neon Theme - Refined Cyberpunk with Pink/Purple/Cyan"""

NEON_CSS = '''
body {
  font-family: 'Sora', system-ui, sans-serif;
  background: #08080f;
  color: #e8e8f0;
}

.page {
  background: linear-gradient(180deg, #0e0e18 0%, #08080f 100%);
  border: 1px solid rgba(139,92,246,0.1);
  box-shadow: 
    0 0 60px rgba(139,92,246,0.04),
    0 0 100px rgba(236,72,153,0.02);
}

.page::after {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0;
  height: 2px;
  background: linear-gradient(90deg, #ec4899, #8b5cf6, #06b6d4);
  z-index: 10;
}

h1 {
  color: #f472b6;
  letter-spacing: 0.03em;
  text-shadow: 
    0 0 30px rgba(236,72,153,0.4),
    0 0 60px rgba(236,72,153,0.2);
}

h2 {
  color: #a78bfa;
  padding: var(--spacing-sm) var(--spacing-md);
  background: linear-gradient(90deg, rgba(139,92,246,0.1) 0%, transparent 70%);
  border-left: 3px solid #8b5cf6;
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  text-align: left;
  text-shadow: 0 0 20px rgba(139,92,246,0.3);
}

h3 { color: #22d3ee; }

h4 {
  color: #c4b5fd;
  font-weight: 500;
}

p { color: #c4c4d0; }
strong { color: #f472b6; }
em { color: #22d3ee; font-style: normal; }
a { color: #ec4899; }
li::marker { color: #8b5cf6; }

/* Tables */
table {
  background: rgba(14,14,24,0.9);
  border: 1px solid rgba(139,92,246,0.2);
  border-radius: var(--radius-md);
  box-shadow: 
    0 4px 24px rgba(0,0,0,0.3),
    0 0 30px rgba(139,92,246,0.05);
}

thead {
  background: linear-gradient(180deg, rgba(139,92,246,0.12) 0%, rgba(139,92,246,0.06) 100%);
}

th {
  color: #f472b6;
  border-bottom: 1px solid rgba(236,72,153,0.3);
  text-shadow: 0 0 10px rgba(236,72,153,0.3);
}

td {
  color: #d4d4e0;
  border-bottom: 1px solid rgba(139,92,246,0.1);
}

tbody tr:nth-child(even) { background: rgba(139,92,246,0.03); }
tbody tr:last-child td { border-bottom: none; }

/* Code */
code {
  background: rgba(6,182,212,0.12);
  color: #22d3ee;
  border: 1px solid rgba(6,182,212,0.2);
}

pre {
  background: rgba(8,8,15,0.95);
  color: #22d3ee;
  border: 1px solid rgba(6,182,212,0.2);
}

blockquote {
  background: rgba(236,72,153,0.06);
  border-left: 3px solid #ec4899;
  color: #c4b5fd;
}

hr {
  background: linear-gradient(90deg, #ec4899, #8b5cf6, #06b6d4);
  box-shadow: 0 0 10px rgba(139,92,246,0.4);
}

img {
  border: 1px solid rgba(139,92,246,0.2);
  box-shadow: 0 4px 24px rgba(0,0,0,0.4);
}

@media print {
  body { background: #08080f !important; }
}
'''
