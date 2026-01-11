"""Gaming Theme - Modern HUD with Green/Red Accents"""

GAMING_CSS = '''
body {
  font-family: 'Sora', system-ui, sans-serif;
  background: #030303;
  color: #e5e5e5;
}

.page {
  background: 
    radial-gradient(ellipse at 50% 0%, rgba(34,197,94,0.02) 0%, transparent 50%),
    linear-gradient(180deg, #0a0a0a 0%, #050505 100%);
  border: 1px solid rgba(34,197,94,0.08);
}

.page::before { opacity: 0.04; }

.page::after {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent 5%, #22c55e 40%, #ef4444 50%, #22c55e 60%, transparent 95%);
  z-index: 10;
}

h1 {
  font-family: 'Orbitron', monospace;
  font-weight: 800;
  color: #22c55e;
  letter-spacing: 0.1em;
  text-shadow: 0 0 30px rgba(34,197,94,0.35);
}

h2 {
  font-family: 'Orbitron', monospace;
  font-weight: 600;
  color: #ef4444;
  letter-spacing: 0.04em;
  padding: var(--spacing-sm) var(--spacing-md);
  background: linear-gradient(90deg, rgba(239,68,68,0.08) 0%, transparent 70%);
  border-left: 3px solid #ef4444;
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  text-align: left;
}

h3 {
  font-family: 'Orbitron', monospace;
  font-weight: 500;
  color: #ffffff;
  padding: var(--spacing-xs) var(--spacing-sm);
  background: rgba(17,17,17,0.8);
  border-left: 2px solid #22c55e;
  display: inline-block;
}

h4 {
  color: #6b7280;
  font-weight: 500;
}

p { color: #d4d4d4; }
strong { color: #22c55e; }
em { color: #fbbf24; font-style: normal; }
a { color: #22c55e; }
li::marker { color: #22c55e; }

/* Tables - HUD Style */
table {
  background: rgba(10,10,10,0.95);
  border: 1px solid rgba(34,197,94,0.15);
  border-radius: var(--radius-md);
  box-shadow: 
    0 4px 24px rgba(0,0,0,0.4),
    0 0 20px rgba(34,197,94,0.03);
}

thead {
  background: linear-gradient(180deg, rgba(34,197,94,0.1) 0%, rgba(34,197,94,0.04) 100%);
}

th {
  font-family: 'Orbitron', monospace;
  color: #22c55e;
  border-bottom: 1px solid rgba(34,197,94,0.3);
  text-shadow: 0 0 8px rgba(34,197,94,0.2);
  font-size: 7pt;
}

td {
  color: #e5e5e5;
  border-bottom: 1px solid rgba(34,197,94,0.06);
}

tbody tr:nth-child(even) { background: rgba(255,255,255,0.015); }
tbody tr:last-child td { border-bottom: none; }

td strong { color: #22c55e; }
td em { color: #fbbf24; }

/* Code */
code {
  background: rgba(34,197,94,0.1);
  color: #22c55e;
  border: 1px solid rgba(34,197,94,0.15);
}

pre {
  background: rgba(5,5,5,0.95);
  color: #22c55e;
  border-left: 2px solid #22c55e;
}

blockquote {
  background: rgba(239,68,68,0.05);
  border-left: 3px solid #ef4444;
  color: #9ca3af;
}

hr {
  background: linear-gradient(90deg, transparent, #22c55e 40%, #ef4444 50%, #22c55e 60%, transparent);
  box-shadow: 0 0 8px rgba(34,197,94,0.3);
}

img {
  border: 1px solid rgba(34,197,94,0.12);
  box-shadow: 0 4px 24px rgba(0,0,0,0.5);
}

@media print {
  body { background: #030303 !important; }
}
'''
