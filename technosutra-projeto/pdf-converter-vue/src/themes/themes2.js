// Minimal Theme - Ultra Clean with subtle dot texture
export const MINIMAL_CSS = `
body { font-family: 'Plus Jakarta Sans', system-ui, sans-serif; background: #ffffff; color: #171717; }
.page { background: #ffffff; border: 1px solid #e5e5e5; }
.page::before {
  background-image: url("data:image/svg+xml,%3Csvg width='20' height='20' xmlns='http://www.w3.org/2000/svg'%3E%3Ccircle cx='10' cy='10' r='0.5' fill='%23000' opacity='0.03'/%3E%3C/svg%3E");
  opacity: 1;
}
.page::after {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'%3E%3Cg fill='none' stroke='%23171717' stroke-width='0.3'%3E%3Ccircle cx='100' cy='100' r='80'/%3E%3Ccircle cx='100' cy='100' r='40'/%3E%3Cpath d='M100 20v160M20 100h160'/%3E%3C/g%3E%3C/svg%3E");
  opacity: 0.02;
}
h1 { color: #0a0a0a; font-weight: 800; }
h2 { color: #171717; border-bottom: 1px solid #e5e5e5; padding-bottom: var(--spacing-sm); text-align: left; }
h3 { color: #262626; }
h4 { color: #525252; }
p { color: #404040; }
strong { color: #0a0a0a; }
em { color: #525252; }
a { color: #171717; text-decoration: underline; }
li::marker { color: #737373; }
table { background: #ffffff; border: 1px solid #e5e5e5; }
thead { background: #fafafa; }
th { color: #171717; border-bottom: 1px solid #d4d4d4; }
td { color: #404040; border-bottom: 1px solid #f5f5f5; }
tbody tr:nth-child(even) { background: #fafafa; }
code { background: #f5f5f5; color: #171717; border: 1px solid #e5e5e5; }
pre { background: #fafafa; color: #171717; border: 1px solid #e5e5e5; }
blockquote { background: #fafafa; border-left: 2px solid #d4d4d4; color: #525252; }
hr { background: #e5e5e5; }
img { border: 1px solid #e5e5e5; }
`

// Luxury Theme - Elegant Gold/Black with ornate texture
export const LUXURY_CSS = `
body { font-family: 'Plus Jakarta Sans', system-ui, sans-serif; background: #0c0a09; color: #e7e5e4; }
.page { background: linear-gradient(180deg, #1c1917 0%, #0c0a09 100%); border: 1px solid rgba(217,169,99,0.2); }
.page::before {
  background-image: 
    url("data:image/svg+xml,%3Csvg width='60' height='60' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M30 0L60 30L30 60L0 30Z' fill='none' stroke='%23d9a963' stroke-width='0.3' opacity='0.08'/%3E%3C/svg%3E"),
    url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.6' numOctaves='4'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
  opacity: 0.06;
}
.page::after {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'%3E%3Cg fill='none' stroke='%23d9a963' stroke-width='0.5'%3E%3Ccircle cx='100' cy='100' r='90'/%3E%3Ccircle cx='100' cy='100' r='70'/%3E%3Ccircle cx='100' cy='100' r='50'/%3E%3Ccircle cx='100' cy='100' r='30'/%3E%3Cpath d='M100 10L130 70L190 100L130 130L100 190L70 130L10 100L70 70Z'/%3E%3Cpath d='M100 30L120 80L170 100L120 120L100 170L80 120L30 100L80 80Z'/%3E%3C/g%3E%3C/svg%3E");
  opacity: 0.05;
}
.page > *:first-child::before {
  content: '';
  position: absolute;
  top: -20mm; left: -22mm; right: -22mm;
  height: 1px;
  background: linear-gradient(90deg, transparent, #d9a963, transparent);
}
h1 { color: #d9a963; font-weight: 700; letter-spacing: 0.05em; text-shadow: 0 0 30px rgba(217,169,99,0.2); }
h2 { color: #d9a963; padding: var(--spacing-sm) var(--spacing-md); border-left: 2px solid #d9a963; text-align: left; }
h3 { color: #fef3c7; }
h4 { color: #a8a29e; }
p { color: #d6d3d1; }
strong { color: #d9a963; }
em { color: #fef3c7; }
a { color: #d9a963; }
li::marker { color: #d9a963; }
table { background: rgba(28,25,23,0.9); border: 1px solid rgba(217,169,99,0.2); }
thead { background: linear-gradient(180deg, rgba(217,169,99,0.1) 0%, rgba(217,169,99,0.05) 100%); }
th { color: #d9a963; border-bottom: 1px solid rgba(217,169,99,0.3); }
td { color: #d6d3d1; border-bottom: 1px solid rgba(217,169,99,0.08); }
tbody tr:nth-child(even) { background: rgba(217,169,99,0.02); }
code { background: rgba(217,169,99,0.1); color: #d9a963; border: 1px solid rgba(217,169,99,0.2); }
pre { background: rgba(12,10,9,0.95); color: #d9a963; border: 1px solid rgba(217,169,99,0.2); }
blockquote { background: rgba(217,169,99,0.05); border-left: 2px solid #d9a963; color: #a8a29e; }
hr { background: linear-gradient(90deg, transparent, #d9a963, transparent); }
img { border: 1px solid rgba(217,169,99,0.2); box-shadow: 0 4px 24px rgba(0,0,0,0.4); }
`

// Nature Theme - Organic Forest Green with leaf texture
export const NATURE_CSS = `
body { font-family: 'Plus Jakarta Sans', system-ui, sans-serif; background: #f0fdf4; color: #14532d; }
.page { background: linear-gradient(180deg, #f0fdf4 0%, #ecfdf5 100%); border: 1px solid #bbf7d0; }
.page::before {
  background-image: 
    url("data:image/svg+xml,%3Csvg width='100' height='100' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M50 10Q70 30 50 50Q30 30 50 10' fill='none' stroke='%2322c55e' stroke-width='0.5' opacity='0.08'/%3E%3C/svg%3E"),
    url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='4'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
  opacity: 0.04;
}
.page::after {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'%3E%3Cg fill='none' stroke='%2322c55e' stroke-width='0.4'%3E%3Ccircle cx='100' cy='100' r='90'/%3E%3Ccircle cx='100' cy='100' r='70'/%3E%3Ccircle cx='100' cy='100' r='50'/%3E%3Ccircle cx='100' cy='100' r='30'/%3E%3Cpath d='M100 10Q140 50 100 100Q60 50 100 10'/%3E%3Cpath d='M100 190Q140 150 100 100Q60 150 100 190'/%3E%3Cpath d='M10 100Q50 140 100 100Q50 60 10 100'/%3E%3Cpath d='M190 100Q150 140 100 100Q150 60 190 100'/%3E%3C/g%3E%3C/svg%3E");
  opacity: 0.04;
}
h1 { color: #14532d; font-weight: 800; }
h2 { color: #166534; padding: var(--spacing-sm) var(--spacing-md); background: linear-gradient(90deg, rgba(22,101,52,0.06) 0%, transparent 70%); border-left: 3px solid #22c55e; text-align: left; }
h3 { color: #15803d; }
h4 { color: #4ade80; }
p { color: #166534; }
strong { color: #14532d; }
em { color: #15803d; }
a { color: #16a34a; }
li::marker { color: #22c55e; }
table { background: #ffffff; border: 1px solid #bbf7d0; }
thead { background: linear-gradient(180deg, #dcfce7 0%, #f0fdf4 100%); }
th { color: #14532d; border-bottom: 2px solid #22c55e; }
td { color: #166534; border-bottom: 1px solid #dcfce7; }
tbody tr:nth-child(even) { background: #f0fdf4; }
code { background: #dcfce7; color: #14532d; border: 1px solid #bbf7d0; }
pre { background: #f0fdf4; color: #14532d; border: 1px solid #bbf7d0; }
blockquote { background: #dcfce7; border-left: 3px solid #22c55e; color: #15803d; }
hr { background: linear-gradient(90deg, transparent, #22c55e, transparent); }
img { border: 1px solid #bbf7d0; box-shadow: 0 4px 12px rgba(0,0,0,0.06); }
`

// Tech Theme - Futuristic Blue with circuit texture
export const TECH_CSS = `
body { font-family: 'Sora', system-ui, sans-serif; background: #020617; color: #e2e8f0; }
.page { background: linear-gradient(180deg, #0f172a 0%, #020617 100%); border: 1px solid rgba(59,130,246,0.15); }
.page::before {
  background-image: 
    url("data:image/svg+xml,%3Csvg width='100' height='100' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M10 50h30M60 50h30M50 10v30M50 60v30' stroke='%233b82f6' stroke-width='0.5' opacity='0.1'/%3E%3Ccircle cx='50' cy='50' r='3' fill='none' stroke='%233b82f6' stroke-width='0.5' opacity='0.1'/%3E%3C/svg%3E"),
    url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.7' numOctaves='3'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
  opacity: 0.06;
}
.page::after {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'%3E%3Cg fill='none' stroke='%233b82f6' stroke-width='0.5'%3E%3Ccircle cx='100' cy='100' r='90'/%3E%3Ccircle cx='100' cy='100' r='70'/%3E%3Ccircle cx='100' cy='100' r='50'/%3E%3Ccircle cx='100' cy='100' r='30'/%3E%3Cpath d='M100 10v180M10 100h180'/%3E%3Cpath d='M30 30L170 170M170 30L30 170'/%3E%3Crect x='70' y='70' width='60' height='60' rx='5'/%3E%3C/g%3E%3C/svg%3E");
  opacity: 0.05;
}
.page > *:first-child::before {
  content: '';
  position: absolute;
  top: -20mm; left: -22mm; right: -22mm;
  height: 2px;
  background: linear-gradient(90deg, transparent, #3b82f6, #06b6d4, transparent);
}
h1 { color: #3b82f6; letter-spacing: 0.03em; text-shadow: 0 0 30px rgba(59,130,246,0.3); }
h2 { color: #60a5fa; padding: var(--spacing-sm) var(--spacing-md); background: linear-gradient(90deg, rgba(59,130,246,0.1) 0%, transparent 70%); border-left: 3px solid #3b82f6; text-align: left; }
h3 { color: #22d3ee; }
h4 { color: #94a3b8; }
p { color: #cbd5e1; }
strong { color: #3b82f6; }
em { color: #22d3ee; font-style: normal; }
a { color: #3b82f6; }
li::marker { color: #3b82f6; }
table { background: rgba(15,23,42,0.9); border: 1px solid rgba(59,130,246,0.2); box-shadow: 0 4px 24px rgba(0,0,0,0.3); }
thead { background: linear-gradient(180deg, rgba(59,130,246,0.12) 0%, rgba(59,130,246,0.06) 100%); }
th { color: #3b82f6; border-bottom: 1px solid rgba(59,130,246,0.3); }
td { color: #e2e8f0; border-bottom: 1px solid rgba(59,130,246,0.1); }
tbody tr:nth-child(even) { background: rgba(59,130,246,0.03); }
code { background: rgba(6,182,212,0.12); color: #22d3ee; border: 1px solid rgba(6,182,212,0.2); }
pre { background: rgba(2,6,23,0.95); color: #22d3ee; border: 1px solid rgba(6,182,212,0.2); }
blockquote { background: rgba(59,130,246,0.06); border-left: 3px solid #3b82f6; color: #94a3b8; }
hr { background: linear-gradient(90deg, transparent, #3b82f6, #06b6d4, transparent); }
img { border: 1px solid rgba(59,130,246,0.2); box-shadow: 0 4px 24px rgba(0,0,0,0.4); }
`

// Classic Theme - Traditional Serif/Paper with parchment texture
export const CLASSIC_CSS = `
body { font-family: 'Georgia', 'Times New Roman', serif; background: #fffbeb; color: #292524; }
.page { background: linear-gradient(180deg, #fffbeb 0%, #fef3c7 100%); border: 1px solid #fde68a; }
.page::before {
  background-image: 
    url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.5' numOctaves='5'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
  opacity: 0.05;
}
.page::after {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'%3E%3Cg fill='none' stroke='%23f59e0b' stroke-width='0.4'%3E%3Ccircle cx='100' cy='100' r='90'/%3E%3Ccircle cx='100' cy='100' r='70'/%3E%3Ccircle cx='100' cy='100' r='50'/%3E%3Ccircle cx='100' cy='100' r='30'/%3E%3Cpath d='M100 10v180M10 100h180'/%3E%3Cpath d='M100 10L150 50L190 100L150 150L100 190L50 150L10 100L50 50Z'/%3E%3C/g%3E%3C/svg%3E");
  opacity: 0.04;
}
h1 { color: #78350f; font-weight: 700; font-family: 'Georgia', serif; }
h2 { color: #92400e; font-family: 'Georgia', serif; border-bottom: 2px solid #f59e0b; padding-bottom: var(--spacing-sm); text-align: left; }
h3 { color: #a16207; font-family: 'Georgia', serif; }
h4 { color: #b45309; }
p { color: #44403c; }
strong { color: #78350f; }
em { color: #92400e; }
a { color: #b45309; }
li::marker { color: #f59e0b; }
table { background: #fffbeb; border: 1px solid #fde68a; }
thead { background: #fef3c7; }
th { color: #78350f; border-bottom: 2px solid #f59e0b; font-family: 'Georgia', serif; }
td { color: #44403c; border-bottom: 1px solid #fde68a; }
tbody tr:nth-child(even) { background: #fef3c7; }
code { background: #fef3c7; color: #78350f; border: 1px solid #fde68a; font-family: 'Courier New', monospace; }
pre { background: #fef3c7; color: #44403c; border: 1px solid #fde68a; }
blockquote { background: #fef3c7; border-left: 3px solid #f59e0b; color: #92400e; font-style: italic; }
hr { background: linear-gradient(90deg, transparent, #f59e0b, transparent); }
img { border: 1px solid #fde68a; box-shadow: 0 2px 8px rgba(0,0,0,0.08); }
`
