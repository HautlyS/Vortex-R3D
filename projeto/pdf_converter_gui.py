#!/usr/bin/env python3
"""
TECHNO SUTRA - PDF Converter v6.0
9 Professional Themes | Smart Page Breaks | A4 Preview Simulation
"""

import sys, os, re, subprocess
from pathlib import Path

from PyQt6.QtWidgets import (
    QApplication, QMainWindow, QWidget, QVBoxLayout, QHBoxLayout,
    QLabel, QPushButton, QListWidget, QListWidgetItem, QComboBox,
    QFileDialog, QSplitter, QGroupBox, QSlider, QMessageBox,
    QPlainTextEdit, QSizeGrip
)
from PyQt6.QtCore import Qt, QThread, pyqtSignal, QTimer, QUrl, QPoint
from PyQt6.QtGui import QFont, QColor, QSyntaxHighlighter, QTextCharFormat

try:
    from PyQt6.QtWebEngineWidgets import QWebEngineView
    from PyQt6.QtWebEngineCore import QWebEngineSettings
    HAS_WEBENGINE = True
except ImportError:
    HAS_WEBENGINE = False

# Add themes to path
sys.path.insert(0, str(Path(__file__).parent))
from themes.manager import THEMES, get_css, get_theme_names
from preview_engine import generate_preview_html, generate_pdf_html

SCRIPT_DIR = Path(__file__).parent
DOCS_DIR = SCRIPT_DIR / "documentos"
TEMPLATES_DIR = SCRIPT_DIR / "templates"
ANEXOS_DIR = SCRIPT_DIR / "anexos"
OUTPUT_DIR = SCRIPT_DIR / "pdf_output"


class PreviewThread(QThread):
    html_ready = pyqtSignal(str)
    
    def __init__(self, markdown: str, css: str, base_path: str = ""):
        super().__init__()
        self.markdown = markdown
        self.css = css
        self.base_path = base_path
    
    def run(self):
        try:
            result = subprocess.run([
                'pandoc', '-f', 'markdown+pipe_tables+strikeout+emoji',
                '-t', 'html5', '--wrap=none'
            ], input=self.markdown, capture_output=True, text=True, timeout=30)
            
            if result.returncode != 0:
                self.html_ready.emit(f'<body style="background:#111;color:#f00;padding:40px"><pre>Pandoc error: {result.stderr}</pre></body>')
                return
            
            body_content = result.stdout if result.stdout.strip() else '<p>Empty content</p>'
            
            # Use preview engine for A4 page simulation
            html = generate_preview_html(body_content, self.css, self.base_path)
            self.html_ready.emit(html)
        except subprocess.TimeoutExpired:
            self.html_ready.emit('<body style="background:#111;color:#f00;padding:40px"><pre>Pandoc timeout</pre></body>')
        except FileNotFoundError:
            self.html_ready.emit('<body style="background:#111;color:#f00;padding:40px"><pre>Pandoc not found. Install with: sudo apt install pandoc</pre></body>')
        except Exception as e:
            self.html_ready.emit(f'<body style="background:#111;color:#f00;padding:40px"><pre>Error: {type(e).__name__}: {e}</pre></body>')


class ConvertThread(QThread):
    progress = pyqtSignal(str)
    finished = pyqtSignal(bool, str)
    
    def __init__(self, files, css, base_path):
        super().__init__()
        self.files = files
        self.css = css
        self.base_path = base_path
    
    def run(self):
        OUTPUT_DIR.mkdir(exist_ok=True)
        converted = 0
        
        for f in self.files:
            name = Path(f).stem
            self.progress.emit(f"Convertendo: {name}")
            
            try:
                md_content = Path(f).read_text(encoding='utf-8')
                
                result = subprocess.run([
                    'pandoc', '-f', 'markdown+pipe_tables+strikeout',
                    '-t', 'html5', '--wrap=none'
                ], input=md_content, capture_output=True, text=True)
                
                # Use PDF-optimized HTML (no preview wrapper)
                html_content = generate_pdf_html(result.stdout, self.css, str(Path(f).parent))
                
                html_path = OUTPUT_DIR / f"{name}.html"
                pdf_path = OUTPUT_DIR / f"{name}.pdf"
                html_path.write_text(html_content, encoding='utf-8')
                
                subprocess.run([
                    'wkhtmltopdf', '--quiet', '--enable-local-file-access',
                    '--page-size', 'A4', '--orientation', 'Portrait',
                    '--margin-top', '0', '--margin-bottom', '0',
                    '--margin-left', '0', '--margin-right', '0',
                    '--dpi', '300', '--print-media-type', '--no-outline',
                    '--disable-smart-shrinking',
                    str(html_path), str(pdf_path)
                ], capture_output=True)
                
                if pdf_path.exists():
                    converted += 1
                    
            except Exception as e:
                self.progress.emit(f"Erro: {name} - {e}")
        
        self.finished.emit(True, f"{converted}/{len(self.files)} PDFs gerados em pdf_output/")


class MarkdownHighlighter(QSyntaxHighlighter):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.rules = []
        
        fmt = QTextCharFormat()
        fmt.setForeground(QColor("#22c55e"))
        fmt.setFontWeight(QFont.Weight.Bold)
        self.rules.append((re.compile(r'^#{1,6}\s.+$', re.MULTILINE), fmt))
        
        fmt = QTextCharFormat()
        fmt.setFontWeight(QFont.Weight.Bold)
        fmt.setForeground(QColor("#ffffff"))
        self.rules.append((re.compile(r'\*\*[^*]+\*\*'), fmt))
        
        fmt = QTextCharFormat()
        fmt.setFontFamily("JetBrains Mono")
        fmt.setForeground(QColor("#22c55e"))
        self.rules.append((re.compile(r'`[^`]+`'), fmt))
        
        fmt = QTextCharFormat()
        fmt.setForeground(QColor("#ef4444"))
        self.rules.append((re.compile(r'\[([^\]]+)\]\([^)]+\)'), fmt))
        
        fmt = QTextCharFormat()
        fmt.setForeground(QColor("#a3a3a3"))
        self.rules.append((re.compile(r'^\|.+\|$', re.MULTILINE), fmt))
    
    def highlightBlock(self, text):
        for pattern, fmt in self.rules:
            for match in pattern.finditer(text):
                self.setFormat(match.start(), match.end() - match.start(), fmt)


class TitleBar(QWidget):
    def __init__(self, parent):
        super().__init__(parent)
        self.parent = parent
        self.dragging = False
        self.drag_pos = QPoint()
        self.setFixedHeight(44)
        
        layout = QHBoxLayout(self)
        layout.setContentsMargins(16, 0, 16, 0)
        layout.setSpacing(10)
        
        for color, slot in [("#ff5f57", parent.close), ("#febc2e", parent.showMinimized), ("#28c840", self._toggle_max)]:
            btn = QPushButton()
            btn.setFixedSize(14, 14)
            btn.setStyleSheet(f"background:{color};border:none;border-radius:7px;")
            btn.clicked.connect(slot)
            layout.addWidget(btn)
        
        layout.addSpacing(20)
        
        title = QLabel("⚡ TECHNO SUTRA PDF v6.0")
        title.setStyleSheet("color:#22c55e;font-family:'Orbitron',monospace;font-size:13px;font-weight:700;letter-spacing:2px;")
        layout.addWidget(title)
        
        layout.addStretch()
        
        self.status = QLabel("● READY")
        self.status.setStyleSheet("color:#525252;font-size:10px;font-weight:600;")
        layout.addWidget(self.status)
    
    def _toggle_max(self):
        if self.parent.isMaximized(): self.parent.showNormal()
        else: self.parent.showMaximized()
    
    def set_status(self, text, color="#22c55e"):
        self.status.setText(f"● {text.upper()}")
        self.status.setStyleSheet(f"color:{color};font-size:10px;font-weight:600;")
    
    def mousePressEvent(self, e):
        if e.button() == Qt.MouseButton.LeftButton:
            self.dragging = True
            self.drag_pos = e.globalPosition().toPoint() - self.parent.frameGeometry().topLeft()
    
    def mouseMoveEvent(self, e):
        if self.dragging:
            self.parent.move(e.globalPosition().toPoint() - self.drag_pos)
    
    def mouseReleaseEvent(self, e):
        self.dragging = False
    
    def mouseDoubleClickEvent(self, e):
        self._toggle_max()


class EditorWidget(QPlainTextEdit):
    content_changed = pyqtSignal()
    
    def __init__(self):
        super().__init__()
        self.highlighter = MarkdownHighlighter(self.document())
        self.setFont(QFont("JetBrains Mono", 11))
        self.textChanged.connect(self.content_changed.emit)


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowFlags(Qt.WindowType.FramelessWindowHint)
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)
        self.setMinimumSize(1500, 900)
        
        self.current_file = None
        self.current_theme = "🎮 Gaming"
        self.preview_timer = QTimer()
        self.preview_timer.setSingleShot(True)
        self.preview_timer.timeout.connect(self._update_preview)
        
        self._setup_ui()
        self._load_files()
    
    def _setup_ui(self):
        container = QWidget()
        container.setObjectName("main")
        self.setCentralWidget(container)
        
        layout = QVBoxLayout(container)
        layout.setContentsMargins(2, 2, 2, 2)
        layout.setSpacing(0)
        
        self.title_bar = TitleBar(self)
        layout.addWidget(self.title_bar)
        
        content = QWidget()
        content_layout = QVBoxLayout(content)
        content_layout.setContentsMargins(12, 8, 12, 12)
        content_layout.setSpacing(10)
        
        toolbar = self._create_toolbar()
        content_layout.addWidget(toolbar)
        
        splitter = QSplitter(Qt.Orientation.Horizontal)
        splitter.setHandleWidth(3)
        
        splitter.addWidget(self._create_file_panel())
        splitter.addWidget(self._create_editor_panel())
        splitter.addWidget(self._create_preview_panel())
        
        splitter.setSizes([220, 450, 700])
        content_layout.addWidget(splitter)
        
        layout.addWidget(content)
        self._apply_theme()
        QSizeGrip(self)
    
    def _create_toolbar(self):
        toolbar = QWidget()
        toolbar.setObjectName("toolbar")
        toolbar.setFixedHeight(56)
        
        layout = QHBoxLayout(toolbar)
        layout.setContentsMargins(12, 8, 12, 8)
        layout.setSpacing(10)
        
        # Theme selector
        theme_label = QLabel("TEMA:")
        theme_label.setStyleSheet("color:#22c55e;font-weight:700;font-size:11px;")
        layout.addWidget(theme_label)
        
        self.theme_combo = QComboBox()
        self.theme_combo.addItems(get_theme_names())
        self.theme_combo.setCurrentText(self.current_theme)
        self.theme_combo.currentTextChanged.connect(self._on_theme_changed)
        self.theme_combo.setFixedWidth(160)
        layout.addWidget(self.theme_combo)
        
        self.theme_desc = QLabel(THEMES[self.current_theme]["desc"])
        self.theme_desc.setStyleSheet("color:#737373;font-size:10px;font-style:italic;")
        layout.addWidget(self.theme_desc)
        
        layout.addSpacing(20)
        
        buttons = [
            ("📂 ABRIR", self._open_file, "#262626"),
            ("💾 SALVAR", self._save_file, "#262626"),
            ("👁️ PREVIEW", self._update_preview, "#166534"),
            ("📄 PDF", self._convert_current, "#991b1b"),
            ("📚 TODOS", self._convert_all, "#854d0e"),
        ]
        
        for text, slot, bg in buttons:
            btn = QPushButton(text)
            btn.setStyleSheet(f"""
                QPushButton {{
                    background: {bg}; border: 1px solid #404040;
                    border-radius: 6px; padding: 8px 16px;
                    color: #e5e5e5; font-weight: 600; font-size: 11px;
                }}
                QPushButton:hover {{ background: #404040; border-color: #22c55e; }}
            """)
            btn.clicked.connect(slot)
            layout.addWidget(btn)
        
        layout.addStretch()
        return toolbar
    
    def _create_file_panel(self):
        panel = QWidget()
        layout = QVBoxLayout(panel)
        layout.setContentsMargins(0, 0, 0, 0)
        
        group = QGroupBox("◈ ARQUIVOS")
        group_layout = QVBoxLayout(group)
        
        self.file_list = QListWidget()
        self.file_list.itemClicked.connect(self._on_file_clicked)
        group_layout.addWidget(self.file_list)
        
        layout.addWidget(group)
        return panel
    
    def _create_editor_panel(self):
        panel = QWidget()
        layout = QVBoxLayout(panel)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(6)
        
        header = QWidget()
        header.setObjectName("panelHeader")
        header_layout = QHBoxLayout(header)
        header_layout.setContentsMargins(12, 8, 12, 8)
        
        self.editor_label = QLabel("◉ EDITOR")
        self.editor_label.setStyleSheet("color:#fbbf24;font-weight:700;font-size:11px;letter-spacing:1px;")
        header_layout.addWidget(self.editor_label)
        header_layout.addStretch()
        
        self.line_info = QLabel("L:1 C:1")
        self.line_info.setStyleSheet("color:#525252;font-size:10px;")
        header_layout.addWidget(self.line_info)
        
        layout.addWidget(header)
        
        self.editor = EditorWidget()
        self.editor.content_changed.connect(self._on_content_changed)
        self.editor.cursorPositionChanged.connect(self._update_cursor)
        layout.addWidget(self.editor)
        
        return panel
    
    def _create_preview_panel(self):
        panel = QWidget()
        layout = QVBoxLayout(panel)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(6)
        
        header = QWidget()
        header.setObjectName("panelHeader")
        header_layout = QHBoxLayout(header)
        header_layout.setContentsMargins(12, 8, 12, 8)
        
        lbl = QLabel("◉ PREVIEW PDF")
        lbl.setStyleSheet("color:#22c55e;font-weight:700;font-size:11px;letter-spacing:1px;")
        header_layout.addWidget(lbl)
        header_layout.addStretch()
        
        header_layout.addWidget(QLabel("ZOOM"))
        self.zoom_slider = QSlider(Qt.Orientation.Horizontal)
        self.zoom_slider.setRange(50, 150)
        self.zoom_slider.setValue(80)
        self.zoom_slider.setFixedWidth(80)
        self.zoom_slider.valueChanged.connect(self._on_zoom)
        header_layout.addWidget(self.zoom_slider)
        
        self.zoom_label = QLabel("80%")
        self.zoom_label.setStyleSheet("color:#22c55e;font-size:10px;min-width:30px;")
        header_layout.addWidget(self.zoom_label)
        
        layout.addWidget(header)
        
        if HAS_WEBENGINE:
            self.preview = QWebEngineView()
            self.preview.setZoomFactor(0.8)
            settings = self.preview.settings()
            settings.setAttribute(QWebEngineSettings.WebAttribute.LocalContentCanAccessFileUrls, True)
            settings.setAttribute(QWebEngineSettings.WebAttribute.LocalContentCanAccessRemoteUrls, True)
            settings.setAttribute(QWebEngineSettings.WebAttribute.JavascriptEnabled, True)
            settings.setAttribute(QWebEngineSettings.WebAttribute.LocalStorageEnabled, True)
        else:
            from PyQt6.QtWidgets import QTextBrowser
            self.preview = QTextBrowser()
        
        layout.addWidget(self.preview)
        return panel
    
    def _apply_theme(self):
        self.setStyleSheet('''
            #main {
                background: #0a0a0a;
                border: 2px solid #1f1f1f;
                border-radius: 12px;
            }
            #toolbar {
                background: #141414;
                border: 1px solid #1f1f1f;
                border-radius: 8px;
            }
            #panelHeader {
                background: #141414;
                border: 1px solid #1f1f1f;
                border-radius: 8px;
            }
            QGroupBox {
                background: #141414;
                border: 1px solid #1f1f1f;
                border-radius: 10px;
                margin-top: 14px;
                padding: 10px;
                padding-top: 22px;
                font-weight: 700;
                color: #22c55e;
                font-size: 10px;
            }
            QGroupBox::title {
                subcontrol-origin: margin;
                left: 12px; top: 3px;
            }
            QListWidget {
                background: #0a0a0a;
                border: 1px solid #1f1f1f;
                border-radius: 8px;
                color: #d4d4d4;
                font-size: 11px;
            }
            QListWidget::item {
                padding: 10px;
                border-radius: 6px;
                margin: 2px;
            }
            QListWidget::item:selected {
                background: #166534;
                color: #ffffff;
            }
            QListWidget::item:hover:!selected {
                background: #1f1f1f;
            }
            QPlainTextEdit {
                background: #0a0a0a;
                border: 1px solid #1f1f1f;
                border-radius: 8px;
                color: #e5e5e5;
                padding: 12px;
                selection-background-color: #166534;
            }
            QComboBox {
                background: #1a1a1a;
                border: 1px solid #333;
                border-radius: 6px;
                padding: 6px 12px;
                color: #e5e5e5;
                font-weight: 600;
                font-size: 11px;
            }
            QComboBox:hover { border-color: #22c55e; }
            QComboBox::drop-down {
                border: none;
                width: 20px;
            }
            QComboBox QAbstractItemView {
                background: #1a1a1a;
                border: 1px solid #333;
                color: #e5e5e5;
                selection-background-color: #166534;
            }
            QSlider::groove:horizontal {
                height: 4px;
                background: #262626;
                border-radius: 2px;
            }
            QSlider::handle:horizontal {
                width: 14px; height: 14px;
                background: #22c55e;
                border-radius: 7px;
                margin: -5px 0;
            }
            QScrollBar:vertical {
                background: #0a0a0a;
                width: 8px;
                border-radius: 4px;
            }
            QScrollBar::handle:vertical {
                background: #262626;
                border-radius: 4px;
                min-height: 30px;
            }
            QScrollBar::handle:vertical:hover { background: #22c55e; }
            QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical { height: 0; }
            QLabel { color: #737373; font-size: 10px; }
            QSplitter::handle { background: #1f1f1f; border-radius: 2px; }
            QSplitter::handle:hover { background: #22c55e; }
        ''')
    
    def _load_files(self):
        self.file_list.clear()
        for d in [DOCS_DIR, TEMPLATES_DIR, ANEXOS_DIR, SCRIPT_DIR]:
            if d.exists():
                for f in sorted(d.glob("*.md")):
                    item = QListWidgetItem(f"📄 {f.name}")
                    item.setData(Qt.ItemDataRole.UserRole, str(f))
                    self.file_list.addItem(item)
        self.title_bar.set_status(f"{self.file_list.count()} ARQUIVOS")
    
    def _on_theme_changed(self, theme_name):
        self.current_theme = theme_name
        self.theme_desc.setText(THEMES[theme_name]["desc"])
        self._update_preview()
    
    def _on_file_clicked(self, item):
        path = item.data(Qt.ItemDataRole.UserRole)
        self.current_file = path
        content = Path(path).read_text(encoding='utf-8')
        self.editor.blockSignals(True)
        self.editor.setPlainText(content)
        self.editor.blockSignals(False)
        self.editor_label.setText(f"◉ {Path(path).name}")
        self._update_preview()
    
    def _on_content_changed(self):
        self.preview_timer.start(400)
    
    def _update_cursor(self):
        c = self.editor.textCursor()
        self.line_info.setText(f"L:{c.blockNumber()+1} C:{c.columnNumber()+1}")
    
    def _update_preview(self):
        content = self.editor.toPlainText()
        if not content.strip():
            return
        
        base = str(Path(self.current_file).parent) if self.current_file else str(SCRIPT_DIR)
        css = get_css(self.current_theme)
        
        self.preview_thread = PreviewThread(content, css, base)
        self.preview_thread.html_ready.connect(self._on_preview_ready)
        self.preview_thread.start()
        self.title_bar.set_status("RENDERING...", "#fbbf24")
    
    def _on_preview_ready(self, html):
        if HAS_WEBENGINE:
            # Use setHtml with baseUrl for local file access
            base = Path(self.current_file).parent if self.current_file else SCRIPT_DIR
            base_url = QUrl.fromLocalFile(str(base.resolve()) + '/')
            self.preview.setHtml(html, base_url)
        else:
            self.preview.setHtml(html)
        self.title_bar.set_status("READY", "#22c55e")
    
    def _on_zoom(self, value):
        self.zoom_label.setText(f"{value}%")
        if HAS_WEBENGINE:
            self.preview.setZoomFactor(value / 100)
    
    def _open_file(self):
        f, _ = QFileDialog.getOpenFileName(self, "Abrir", str(DOCS_DIR), "Markdown (*.md)")
        if f:
            self.current_file = f
            self.editor.setPlainText(Path(f).read_text(encoding='utf-8'))
            self._update_preview()
    
    def _save_file(self):
        if not self.current_file:
            f, _ = QFileDialog.getSaveFileName(self, "Salvar", str(DOCS_DIR), "Markdown (*.md)")
            if f: self.current_file = f
            else: return
        Path(self.current_file).write_text(self.editor.toPlainText(), encoding='utf-8')
        self.title_bar.set_status("SAVED", "#22c55e")
    
    def _convert_current(self):
        if not self.current_file:
            QMessageBox.warning(self, "Aviso", "Selecione um arquivo")
            return
        self._run_convert([self.current_file])
    
    def _convert_all(self):
        files = [self.file_list.item(i).data(Qt.ItemDataRole.UserRole) for i in range(self.file_list.count())]
        if files:
            self._run_convert(files)
    
    def _run_convert(self, files):
        self.title_bar.set_status("CONVERTING...", "#fbbf24")
        css = get_css(self.current_theme)
        self.convert_thread = ConvertThread(files, css, str(SCRIPT_DIR))
        self.convert_thread.progress.connect(lambda m: self.title_bar.set_status(m, "#fbbf24"))
        self.convert_thread.finished.connect(self._on_convert_done)
        self.convert_thread.start()
    
    def _on_convert_done(self, ok, msg):
        self.title_bar.set_status("DONE", "#22c55e")
        QMessageBox.information(self, "Conversão", msg)


def main():
    app = QApplication(sys.argv)
    app.setStyle('Fusion')
    window = MainWindow()
    window.show()
    sys.exit(app.exec())


if __name__ == "__main__":
    main()
