"""Luxury Theme - Premium Black & Gold with Sophisticated Feel"""

LUXURY_CSS = '''
@import url('https://fonts.googleapis.com/css2?family=Cormorant+Garamond:wght@400;500;600;700&display=swap');

body {
  font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
  background: #030303;
  color: #e8e8e8;
}

.page {
  background: linear-gradient(180deg, #0c0c0c 0%, #050505 100%);
  border: 1px solid rgba(212,175,55,0.1);
  box-shadow: 
    0 0 80px rgba(212,175,55,0.03),
    inset 0 1px 0 rgba(212,175,55,0.08);
}

.page::after {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent 10%, #a08030 30%, #d4af37 50%, #a08030 70%, transparent 90%);
  z-index: 10;
}

h1 {
  font-family: 'Cormorant Garamond', Georgia, serif;
  font-weight: 600;
  color: #d4af37;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  text-shadow: 0 0 30px rgba(212,175,55,0.25);
}

h2 {
  font-family: 'Cormorant Garamond', Georgia, serif;
  font-weight: 500;
  color: #e8d5a3;
  letter-spacing: 0.04em;
  padding-bottom: var(--spacing-sm);
  border-bottom: 1px solid rgba(212,175,55,0.2);
}

h3 {
  color: #d4af37;
  font-weight: 600;
}

h4 {
  color: #a08030;
  font-weight: 500;
  letter-spacing: 0.15em;
}

p { color: #c4c4c4; }
strong { color: #d4af37; }
em { color: #e8d5a3; }
a { color: #d4af37; }
li::marker { color: #d4af37; }

/* Tables */
table {
  background: rgba(12,12,12,0.9);
  border: 1px solid rgba(212,175,55,0.15);
  border-radius: var(--radius-md);
  box-shadow: 0 8px 32px rgba(0,0,0,0.4);
}

thead {
  background: linear-gradient(180deg, rgba(212,175,55,0.1) 0%, rgba(212,175,55,0.04) 100%);
}

th {
  color: #d4af37;
  border-bottom: 1px solid rgba(212,175,55,0.3);
  letter-spacing: 0.1em;
}

td {
  color: #d4d4d4;
  border-bottom: 1px solid rgba(212,175,55,0.06);
}

tbody tr:nth-child(even) { background: rgba(212,175,55,0.02); }
tbody tr:last-child td { border-bottom: none; }

/* Code */
code {
  background: rgba(212,175,55,0.1);
  color: #e8d5a3;
  border: 1px solid rgba(212,175,55,0.15);
}

pre {
  background: rgba(5,5,5,0.95);
  color: #d4af37;
  border: 1px solid rgba(212,175,55,0.12);
}

blockquote {
  background: rgba(212,175,55,0.04);
  border-left: 2px solid #d4af37;
  color: #a08030;
}

hr {
  background: linear-gradient(90deg, transparent, #d4af37, transparent);
}

img {
  border: 1px solid rgba(212,175,55,0.12);
  box-shadow: 0 8px 32px rgba(0,0,0,0.5);
}

@media print {
  body { background: #050505 !important; }
}
'''
