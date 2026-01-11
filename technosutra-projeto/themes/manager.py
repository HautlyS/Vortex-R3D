"""Theme Manager - 9 Professional PDF Themes"""

from .base import BASE_CSS
from .gaming import GAMING_CSS
from .corporate import CORPORATE_CSS
from .zen import ZEN_CSS
from .neon import NEON_CSS
from .minimal import MINIMAL_CSS
from .luxury import LUXURY_CSS
from .nature import NATURE_CSS
from .tech import TECH_CSS
from .classic import CLASSIC_CSS

THEMES = {
    "🎮 Gaming": {"css": BASE_CSS + GAMING_CSS, "desc": "Cyberpunk HUD - Verde/Vermelho"},
    "🏢 Corporate": {"css": BASE_CSS + CORPORATE_CSS, "desc": "Profissional - Azul/Branco"},
    "🧘 Zen": {"css": BASE_CSS + ZEN_CSS, "desc": "Minimalista - Tons Terrosos"},
    "💜 Neon": {"css": BASE_CSS + NEON_CSS, "desc": "Cyberpunk - Rosa/Roxo/Cyan"},
    "⬜ Minimal": {"css": BASE_CSS + MINIMAL_CSS, "desc": "Ultra Clean - Preto/Branco"},
    "👑 Luxury": {"css": BASE_CSS + LUXURY_CSS, "desc": "Elegante - Dourado/Preto"},
    "🌿 Nature": {"css": BASE_CSS + NATURE_CSS, "desc": "Orgânico - Verde Floresta"},
    "🔷 Tech": {"css": BASE_CSS + TECH_CSS, "desc": "Futurista - Azul Tech"},
    "📜 Classic": {"css": BASE_CSS + CLASSIC_CSS, "desc": "Tradicional - Serif/Papel"},
}

def get_theme(name: str) -> dict:
    return THEMES.get(name, THEMES["🎮 Gaming"])

def get_theme_names() -> list:
    return list(THEMES.keys())

def get_css(name: str) -> str:
    return get_theme(name)["css"]
