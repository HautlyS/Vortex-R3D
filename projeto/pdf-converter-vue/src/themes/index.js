import { BASE_CSS } from './base.js'
import { GAMING_CSS, CORPORATE_CSS, ZEN_CSS, NEON_CSS } from './themes1.js'
import { MINIMAL_CSS, LUXURY_CSS, NATURE_CSS, TECH_CSS, CLASSIC_CSS } from './themes2.js'

export const THEMES = {
  '🎮 Gaming': { css: BASE_CSS + GAMING_CSS, desc: 'Cyberpunk HUD - Verde/Vermelho', bg: '#030303' },
  '🏢 Corporate': { css: BASE_CSS + CORPORATE_CSS, desc: 'Profissional - Azul/Branco', bg: '#ffffff' },
  '🧘 Zen': { css: BASE_CSS + ZEN_CSS, desc: 'Minimalista - Tons Terrosos', bg: '#faf8f5' },
  '💜 Neon': { css: BASE_CSS + NEON_CSS, desc: 'Cyberpunk - Rosa/Roxo/Cyan', bg: '#08080f' },
  '⬜ Minimal': { css: BASE_CSS + MINIMAL_CSS, desc: 'Ultra Clean - Preto/Branco', bg: '#ffffff' },
  '👑 Luxury': { css: BASE_CSS + LUXURY_CSS, desc: 'Elegante - Dourado/Preto', bg: '#0c0a09' },
  '🌿 Nature': { css: BASE_CSS + NATURE_CSS, desc: 'Orgânico - Verde Floresta', bg: '#f0fdf4' },
  '🔷 Tech': { css: BASE_CSS + TECH_CSS, desc: 'Futurista - Azul Tech', bg: '#020617' },
  '📜 Classic': { css: BASE_CSS + CLASSIC_CSS, desc: 'Tradicional - Serif/Papel', bg: '#fffbeb' }
}

export const getTheme = (name) => THEMES[name] || THEMES['🎮 Gaming']
export const getThemeNames = () => Object.keys(THEMES)
export const getCSS = (name) => getTheme(name).css
