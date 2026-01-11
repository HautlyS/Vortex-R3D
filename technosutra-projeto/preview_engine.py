"""
Preview Engine - A4 Page Simulation with Visual Page Boundaries
Renders preview exactly like final PDF with page breaks
"""

PREVIEW_CSS = '''
/* Preview background */
html, body {
  background: #1a1a1a !important;
  margin: 0;
  padding: 20px;
}

/* A4 page container */
.page {
  width: 210mm;
  min-height: 297mm;
  margin: 0 auto 30px auto;
  box-shadow: 0 8px 40px rgba(0,0,0,0.6), 0 0 0 1px rgba(255,255,255,0.08);
  border-radius: 2px;
  position: relative;
}

/* Page number */
.page::after {
  content: 'A4 Preview';
  position: absolute;
  bottom: 8mm;
  right: 10mm;
  font-size: 8pt;
  color: rgba(128,128,128,0.4);
  font-family: monospace;
  z-index: 100;
}
'''


def generate_preview_html(body_content: str, css: str, base_path: str) -> str:
    """Generate preview HTML with A4 page styling"""
    # Ensure base_path ends with /
    if base_path and not base_path.endswith('/'):
        base_path = base_path + '/'
    
    return f'''<!DOCTYPE html>
<html lang="pt-BR">
<head>
<meta charset="UTF-8">
<base href="file://{base_path}">
<style>
{css}
{PREVIEW_CSS}
</style>
</head>
<body>
<div class="page">
{body_content}
</div>
</body>
</html>'''


def generate_pdf_html(body_content: str, css: str, base_path: str) -> str:
    """Generate HTML optimized for PDF conversion"""
    if base_path and not base_path.endswith('/'):
        base_path = base_path + '/'
    
    return f'''<!DOCTYPE html>
<html lang="pt-BR">
<head>
<meta charset="UTF-8">
<base href="file://{base_path}">
<style>
{css}
</style>
</head>
<body>
<div class="page">
{body_content}
</div>
</body>
</html>'''
