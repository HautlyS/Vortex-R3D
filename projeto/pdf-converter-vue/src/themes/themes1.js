// Gaming Theme - Cyberpunk HUD Green/Red with texture
export const GAMING_CSS = `
body { font-family: 'Sora', system-ui, sans-serif; background: #030303; color: #e5e5e5; }
.page {
  background: radial-gradient(ellipse at 50% 0%, rgba(34,197,94,0.03) 0%, transparent 50%), linear-gradient(180deg, #0a0a0a 0%, #050505 100%);
  border: 1px solid rgba(34,197,94,0.1);
}
/* Grid texture */
.page::before {
  background-image: 
    url("data:image/svg+xml,%3Csvg width='60' height='60' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M0 0h60v60H0z' fill='none'/%3E%3Cpath d='M0 30h60M30 0v60' stroke='%2322c55e' stroke-width='0.3' opacity='0.15'/%3E%3C/svg%3E"),
    url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='4'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
  opacity: 0.08;
}
/* Cyber mandala */
.page::after {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'%3E%3Cg fill='none' stroke='%2322c55e' stroke-width='0.8'%3E%3Ccircle cx='100' cy='100' r='90'/%3E%3Ccircle cx='100' cy='100' r='70'/%3E%3Ccircle cx='100' cy='100' r='50'/%3E%3Ccircle cx='100' cy='100' r='30'/%3E%3Cpolygon points='100,10 190,100 100,190 10,100'/%3E%3Cpolygon points='100,30 170,100 100,170 30,100'/%3E%3Cpath d='M100 10v180M10 100h180'/%3E%3Cpath d='M30 30L170 170M170 30L30 170'/%3E%3C/g%3E%3C/svg%3E");
  opacity: 0.06;
}
/* Top accent line */
.page > *:first-child::before {
  content: '';
  position: absolute;
  top: -20mm; left: -22mm; right: -22mm;
  height: 2px;
  background: linear-gradient(90deg, transparent 5%, #22c55e 40%, #ef4444 50%, #22c55e 60%, transparent 95%);
}
h1 { font-family: 'Orbitron', monospace; font-weight: 800; color: #22c55e; letter-spacing: 0.1em; text-shadow: 0 0 30px rgba(34,197,94,0.35); }
h2 { font-family: 'Orbitron', monospace; font-weight: 600; color: #ef4444; letter-spacing: 0.04em; padding: var(--spacing-sm) var(--spacing-md); background: linear-gradient(90deg, rgba(239,68,68,0.08) 0%, transparent 70%); border-left: 3px solid #ef4444; border-radius: 0 var(--radius-sm) var(--radius-sm) 0; text-align: left; }
h3 { font-family: 'Orbitron', monospace; font-weight: 500; color: #ffffff; padding: var(--spacing-xs) var(--spacing-sm); background: rgba(17,17,17,0.8); border-left: 2px solid #22c55e; display: inline-block; }
h4 { color: #6b7280; }
p { color: #d4d4d4; }
strong { color: #22c55e; }
em { color: #fbbf24; font-style: normal; }
a { color: #22c55e; }
li::marker { color: #22c55e; }
table { background: rgba(10,10,10,0.95); border: 1px solid rgba(34,197,94,0.15); box-shadow: 0 4px 24px rgba(0,0,0,0.4), 0 0 20px rgba(34,197,94,0.03); }
thead { background: linear-gradient(180deg, rgba(34,197,94,0.1) 0%, rgba(34,197,94,0.04) 100%); }
th { font-family: 'Orbitron', monospace; color: #22c55e; border-bottom: 1px solid rgba(34,197,94,0.3); text-shadow: 0 0 8px rgba(34,197,94,0.2); font-size: 7pt; }
td { color: #e5e5e5; border-bottom: 1px solid rgba(34,197,94,0.06); }
tbody tr:nth-child(even) { background: rgba(255,255,255,0.015); }
tbody tr:last-child td { border-bottom: none; }
code { background: rgba(34,197,94,0.1); color: #22c55e; border: 1px solid rgba(34,197,94,0.15); }
pre { background: rgba(5,5,5,0.95); color: #22c55e; border-left: 2px solid #22c55e; }
blockquote { background: rgba(239,68,68,0.05); border-left: 3px solid #ef4444; color: #9ca3af; }
hr { background: linear-gradient(90deg, transparent, #22c55e 40%, #ef4444 50%, #22c55e 60%, transparent); box-shadow: 0 0 8px rgba(34,197,94,0.3); }
img { border: 1px solid rgba(34,197,94,0.12); box-shadow: 0 4px 24px rgba(0,0,0,0.5); }
`

// Corporate Theme - Professional Blue/White with subtle texture
export const CORPORATE_CSS = `
body { font-family: 'Plus Jakarta Sans', system-ui, sans-serif; background: #ffffff; color: #1e293b; }
.page { background: linear-gradient(180deg, #ffffff 0%, #f8fafc 100%); border: 1px solid #e2e8f0; }
.page::before {
  background-image: url("data:image/svg+xml,%3Csvg width='40' height='40' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M0 0h40v40H0z' fill='none'/%3E%3Ccircle cx='20' cy='20' r='1' fill='%231e40af' opacity='0.05'/%3E%3C/svg%3E");
  opacity: 0.5;
}
.page::after {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'%3E%3Cg fill='none' stroke='%231e40af' stroke-width='0.5'%3E%3Ccircle cx='100' cy='100' r='90'/%3E%3Ccircle cx='100' cy='100' r='60'/%3E%3Ccircle cx='100' cy='100' r='30'/%3E%3Cpath d='M100 10v180M10 100h180'/%3E%3C/g%3E%3C/svg%3E");
  opacity: 0.03;
}
h1 { color: #0f172a; font-weight: 800; }
h2 { color: #1e40af; padding: var(--spacing-sm) var(--spacing-md); background: linear-gradient(90deg, rgba(30,64,175,0.06) 0%, transparent 70%); border-left: 3px solid #1e40af; text-align: left; }
h3 { color: #334155; }
h4 { color: #64748b; }
p { color: #475569; }
strong { color: #1e40af; }
em { color: #0369a1; }
a { color: #1e40af; }
li::marker { color: #1e40af; }
table { background: #ffffff; border: 1px solid #e2e8f0; }
thead { background: linear-gradient(180deg, #f8fafc 0%, #f1f5f9 100%); }
th { color: #1e40af; border-bottom: 2px solid #1e40af; }
td { color: #334155; border-bottom: 1px solid #e2e8f0; }
tbody tr:nth-child(even) { background: #f8fafc; }
code { background: #f1f5f9; color: #1e40af; border: 1px solid #e2e8f0; }
pre { background: #f8fafc; color: #334155; border: 1px solid #e2e8f0; }
blockquote { background: #f0f9ff; border-left: 3px solid #0ea5e9; color: #0369a1; }
hr { background: linear-gradient(90deg, transparent, #1e40af, transparent); }
img { border: 1px solid #e2e8f0; box-shadow: 0 4px 12px rgba(0,0,0,0.08); }
`

// Zen Theme - Minimalist Earthy Tones with organic texture
export const ZEN_CSS = `
body { font-family: 'Plus Jakarta Sans', system-ui, sans-serif; background: #faf8f5; color: #44403c; }
.page { background: linear-gradient(180deg, #faf8f5 0%, #f5f3f0 100%); border: 1px solid #e7e5e4; }
.page::before {
  background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='5'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
  opacity: 0.04;
}
.page::after {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'%3E%3Cg fill='none' stroke='%2378716c' stroke-width='0.3'%3E%3Ccircle cx='100' cy='100' r='95'/%3E%3Ccircle cx='100' cy='100' r='75'/%3E%3Ccircle cx='100' cy='100' r='55'/%3E%3Ccircle cx='100' cy='100' r='35'/%3E%3Ccircle cx='100' cy='100' r='15'/%3E%3Cpath d='M100 5v190M5 100h190'/%3E%3C/g%3E%3C/svg%3E");
  opacity: 0.04;
}
h1 { color: #292524; font-weight: 700; letter-spacing: 0.02em; }
h2 { color: #78716c; padding: var(--spacing-sm) var(--spacing-md); border-left: 2px solid #a8a29e; text-align: left; }
h3 { color: #57534e; }
h4 { color: #78716c; }
p { color: #57534e; }
strong { color: #292524; }
em { color: #78716c; }
a { color: #78716c; }
li::marker { color: #a8a29e; }
table { background: #faf8f5; border: 1px solid #e7e5e4; }
thead { background: #f5f3f0; }
th { color: #57534e; border-bottom: 1px solid #d6d3d1; }
td { color: #57534e; border-bottom: 1px solid #e7e5e4; }
tbody tr:nth-child(even) { background: #f5f3f0; }
code { background: #f5f3f0; color: #57534e; border: 1px solid #e7e5e4; }
pre { background: #f5f3f0; color: #57534e; border: 1px solid #e7e5e4; }
blockquote { background: #f5f3f0; border-left: 2px solid #a8a29e; color: #78716c; }
hr { background: #d6d3d1; }
img { border: 1px solid #e7e5e4; box-shadow: 0 2px 8px rgba(0,0,0,0.04); }
`

// Neon Theme - Cyberpunk Pink/Purple/Cyan with glow texture
export const NEON_CSS = `
body { font-family: 'Sora', system-ui, sans-serif; background: #08080f; color: #e8e8f0; }
.page { background: linear-gradient(180deg, #0e0e18 0%, #08080f 100%); border: 1px solid rgba(139,92,246,0.15); box-shadow: 0 0 60px rgba(139,92,246,0.04), 0 0 100px rgba(236,72,153,0.02); }
.page::before {
  background-image: 
    url("data:image/svg+xml,%3Csvg width='80' height='80' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M0 40h80M40 0v80' stroke='%238b5cf6' stroke-width='0.3' opacity='0.1'/%3E%3C/svg%3E"),
    url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.7' numOctaves='3'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
  opacity: 0.06;
}
.page::after {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'%3E%3Cdefs%3E%3ClinearGradient id='g' x1='0%25' y1='0%25' x2='100%25' y2='100%25'%3E%3Cstop offset='0%25' stop-color='%23ec4899'/%3E%3Cstop offset='50%25' stop-color='%238b5cf6'/%3E%3Cstop offset='100%25' stop-color='%2306b6d4'/%3E%3C/linearGradient%3E%3C/defs%3E%3Cg fill='none' stroke='url(%23g)' stroke-width='0.6'%3E%3Ccircle cx='100' cy='100' r='90'/%3E%3Ccircle cx='100' cy='100' r='70'/%3E%3Ccircle cx='100' cy='100' r='50'/%3E%3Ccircle cx='100' cy='100' r='30'/%3E%3Cpath d='M100 10L190 100L100 190L10 100Z'/%3E%3Cpath d='M100 30L170 100L100 170L30 100Z'/%3E%3Cpath d='M50 50L150 150M150 50L50 150'/%3E%3C/g%3E%3C/svg%3E");
  opacity: 0.06;
}
.page > *:first-child::before {
  content: '';
  position: absolute;
  top: -20mm; left: -22mm; right: -22mm;
  height: 2px;
  background: linear-gradient(90deg, #ec4899, #8b5cf6, #06b6d4);
}
h1 { color: #f472b6; letter-spacing: 0.03em; text-shadow: 0 0 30px rgba(236,72,153,0.4), 0 0 60px rgba(236,72,153,0.2); }
h2 { color: #a78bfa; padding: var(--spacing-sm) var(--spacing-md); background: linear-gradient(90deg, rgba(139,92,246,0.1) 0%, transparent 70%); border-left: 3px solid #8b5cf6; text-align: left; text-shadow: 0 0 20px rgba(139,92,246,0.3); }
h3 { color: #22d3ee; }
h4 { color: #c4b5fd; }
p { color: #c4c4d0; }
strong { color: #f472b6; }
em { color: #22d3ee; font-style: normal; }
a { color: #ec4899; }
li::marker { color: #8b5cf6; }
table { background: rgba(14,14,24,0.9); border: 1px solid rgba(139,92,246,0.2); box-shadow: 0 4px 24px rgba(0,0,0,0.3), 0 0 30px rgba(139,92,246,0.05); }
thead { background: linear-gradient(180deg, rgba(139,92,246,0.12) 0%, rgba(139,92,246,0.06) 100%); }
th { color: #f472b6; border-bottom: 1px solid rgba(236,72,153,0.3); text-shadow: 0 0 10px rgba(236,72,153,0.3); }
td { color: #d4d4e0; border-bottom: 1px solid rgba(139,92,246,0.1); }
tbody tr:nth-child(even) { background: rgba(139,92,246,0.03); }
code { background: rgba(6,182,212,0.12); color: #22d3ee; border: 1px solid rgba(6,182,212,0.2); }
pre { background: rgba(8,8,15,0.95); color: #22d3ee; border: 1px solid rgba(6,182,212,0.2); }
blockquote { background: rgba(236,72,153,0.06); border-left: 3px solid #ec4899; color: #c4b5fd; }
hr { background: linear-gradient(90deg, #ec4899, #8b5cf6, #06b6d4); box-shadow: 0 0 10px rgba(139,92,246,0.4); }
img { border: 1px solid rgba(139,92,246,0.2); box-shadow: 0 4px 24px rgba(0,0,0,0.4); }
`
