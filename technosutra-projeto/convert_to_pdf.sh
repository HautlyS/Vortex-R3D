#!/bin/bash
# ╔══════════════════════════════════════════════════════════════════╗
# ║  TECHNO SUTRA VR - Conversor MD → PDF Profissional               ║
# ║  Estilo: Futurista Holográfico | Normas ABNT                     ║
# ╚══════════════════════════════════════════════════════════════════╝

set -e
cd "$(dirname "$0")"

OUT="pdf_output"
mkdir -p "$OUT"

# CSS Holográfico Inline
CSS='@import url("https://fonts.googleapis.com/css2?family=Orbitron:wght@400;700;900&family=Rajdhani:wght@300;400;500;600;700&family=Space+Mono&display=swap");
:root{--bg:#0a0a0f;--bg2:#12121a;--cyan:#00f5ff;--magenta:#ff00ff;--gold:#ffd700;--text:#e0e0e0;--glow:rgba(0,245,255,0.3);--grad:linear-gradient(135deg,#00f5ff,#ff00ff,#ffd700)}
*{margin:0;padding:0;box-sizing:border-box}
body{font-family:"Rajdhani",Arial,sans-serif;font-size:11pt;line-height:1.5;color:var(--text);background:var(--bg);padding:1.5cm;text-align:justify}
h1{font-family:"Orbitron",sans-serif;font-size:20pt;text-align:center;text-transform:uppercase;letter-spacing:3px;background:var(--grad);-webkit-background-clip:text;-webkit-text-fill-color:transparent;margin:1cm 0;padding:0.5cm}
h2{font-family:"Orbitron",sans-serif;font-size:14pt;color:var(--cyan);text-align:center;text-transform:uppercase;margin:1cm 0 0.5cm;border-bottom:1px solid var(--glow);padding-bottom:0.3cm}
h3{font-size:12pt;color:var(--magenta);margin:0.8cm 0 0.3cm;padding-left:0.4cm;border-left:3px solid var(--magenta)}
h4{font-size:11pt;color:var(--gold);margin:0.5cm 0 0.2cm}
p{text-indent:1cm;margin-bottom:0.4cm}
ul,ol{margin:0.4cm 0 0.4cm 1.2cm}li{margin-bottom:0.2cm}
table{width:100%;border-collapse:collapse;margin:0.8cm 0;background:var(--bg2);border:1px solid var(--glow);font-size:10pt}
th{font-family:"Orbitron",sans-serif;font-size:9pt;color:var(--cyan);padding:0.3cm;text-align:center;border-bottom:2px solid var(--cyan);background:#1a1a2e}
td{padding:0.25cm;text-align:center;border-bottom:1px solid rgba(255,255,255,0.1)}
tr:nth-child(even){background:rgba(0,245,255,0.03)}
code{font-family:"Space Mono",monospace;font-size:9pt;background:#1a1a2e;color:var(--gold);padding:0.1cm 0.2cm;border-radius:2px}
pre{font-family:"Space Mono",monospace;font-size:8pt;background:var(--bg2);color:var(--cyan);padding:0.4cm;margin:0.4cm 0;border:1px solid var(--glow);overflow-x:auto;white-space:pre-wrap}
pre code{background:none;padding:0;color:inherit}
blockquote{margin:0.5cm 1cm;padding:0.4cm;background:var(--bg2);border-left:3px solid #8b5cf6;font-style:italic;color:#a0a0a0}
hr{border:none;height:2px;background:var(--grad);margin:1cm 0;opacity:0.5}
strong{color:var(--gold)}a{color:var(--cyan)}
@media print{body{background:var(--bg)!important}*{-webkit-print-color-adjust:exact!important;print-color-adjust:exact!important}}'

echo -e "\033[36m╔══════════════════════════════════════════════════════════════╗\033[0m"
echo -e "\033[36m║  TECHNO SUTRA - MD → PDF Converter (Holographic Style)       ║\033[0m"
echo -e "\033[36m╚══════════════════════════════════════════════════════════════╝\033[0m"

convert() {
    local f="$1" name=$(basename "$1" .md)
    local html="$OUT/${name}.html" pdf="$OUT/${name}.pdf"
    
    printf "\033[33m▸ %-50s\033[0m" "$name"
    
    echo "<!DOCTYPE html><html lang=\"pt-BR\"><head><meta charset=\"UTF-8\"><style>$CSS</style></head><body>" > "$html"
    pandoc "$f" -f markdown -t html5 >> "$html"
    echo "</body></html>" >> "$html"
    
    wkhtmltopdf --quiet --enable-local-file-access --page-size A4 \
        --margin-top 15 --margin-bottom 15 --margin-left 20 --margin-right 15 \
        --encoding UTF-8 "$html" "$pdf" 2>/dev/null && \
        echo -e "\033[32m ✓\033[0m" || echo -e "\033[31m ✗\033[0m"
}

# Converter todos
for f in documentos/*.md templates/*.md anexos/*.md README.md; do
    [ -f "$f" ] && convert "$f"
done

echo ""
echo -e "\033[32m═══════════════════════════════════════════════════════════════\033[0m"
echo -e "\033[32m  ✓ PDFs gerados em: $OUT/\033[0m"
ls "$OUT"/*.pdf 2>/dev/null | wc -l | xargs -I{} echo -e "\033[32m  Total: {} arquivos\033[0m"
du -sh "$OUT" | awk '{print "  Tamanho: "$1}'
echo -e "\033[32m═══════════════════════════════════════════════════════════════\033[0m"
