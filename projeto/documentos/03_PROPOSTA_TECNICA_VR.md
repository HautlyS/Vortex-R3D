# PROPOSTA TÉCNICA
## TECHNO SUTRA VR - Sabedoria Virtual (Virtual Wisdom)
### Edital FOMENTO CULTSP - PNAB Nº 12/2025 - Módulo III (VR/MR)

---

## 1. SINOPSE

Experiência imersiva em VR que transforma o app AR TECHNO SUTRA (financiado pela Khyentse Foundation) em uma jornada cross-device 3D World VR. O usuário acompanha Sudhana, um jovem em busca da iluminação, viajando de professor em professor através de 112 pontos panorâmicos que conectam São Paulo ao Tibet, Índia e Nepal.

**Tecnologia:** Vortex R3D (Rust + Bevy + 3D VR) - 100% Open Source

---

## 2. CONCEITO

### O Projeto Conecta:

1. **Transformação do App AR existente** (Khyentse Foundation) em versão VR cross-device totalmente refeita
2. **Rota física de importância internacional** - Trilha Cascata (terras raras + água mineral mais radioativa das Américas)
3. **Comunidade tradicional** com vínculo direto (projetos socioculturais, terra natal de membros da staff)
4. **Monitoramento de fauna/flora** + paisagens de campos de altitude, cachoeiras e vales
5. **Democratização de texto difícil** em informações acessíveis e interpretações contemporâneas

### Narrativa

Sudhana busca a iluminação, sendo guiado de mestre em mestre (56 Kalyanamitras). Cada encontro oferece um ensinamento único sobre o caminho do Bodhisattva.

---

## 3. ESPECIFICAÇÕES TÉCNICAS

### 3.1 Pontos de Captura (112 total)

| Tipo | Quantidade | Descrição |
|------|------------|-----------|
| Kalyanamitra | 56 | 1 ponto por personagem |
| Vacuidade | 56 | 1 ponto entre cada personagem (Main Virtual Room) |
| **Total** | **112** | Vídeo/Imagem panorâmica 360° |

### 3.2 Funcionalidades

- **First Person POV View** em todos os 112 pontos
- **Personagens 3D renderizados** (56 Kalyanamitras .glb - rebrand/restyle por CCO e CEO)
- **Book Reader Interface** para leitura VR dos textos
- **PDF Reader automático** integrado (livro, características, infos)
- **HUD completo** para Save, Load, Config
- **Narração por personagem** (áudio .mp3 gravado pela staff)
- **Painel holográfico** com projeções de fauna brasileira

### 3.3 Plataformas

| Plataforma | Status |
|------------|--------|
| Linux | ✅ Demo funcional |
| Windows | ✅ Demo funcional |
| VR (HTC Vive XR Elite) | ✅ Demo funcional |
| WebXR | ⚠️ Parcial → Completo |
| iOS | 🔄 Adicionar suporte |
| Android | 🔄 Adicionar suporte |

### 3.4 Stack Tecnológico

```
Vortex R3D Technology
├── Rust (linguagem)
├── Bevy Engine (game engine)
├── OpenXR (VR)
├── wgpu (gráficos)
└── GitHub Actions (CI/CD)
```

**Repositório:** [github.com/HautlyS/Vortex-R3D](https://github.com/HautlyS/Vortex-R3D)

---

## 4. EQUIPE

| Cargo | Nome | Responsabilidades |
|-------|------|-------------------|
| **CCO (Criatividade)** | Levi | Direção artística, música tema, rebrand dos 56 Kalyanamitras |
| **CCO (Comunicação)** | Isa | Marketing, mídias sociais, divulgação |
| **CEO** | Tupa | Gestão executiva, produção, restyle dos personagens |
| **CTO** | Taric | Arquitetura técnica, desenvolvimento VR, infraestrutura |

**Música Tema:** Strikingly Affect + WALLK - OM MUNI MUNI
- [Instagram](https://www.instagram.com/strikingly.affect/)

---

## 5. CONTRAPARTIDA ECOLÓGICA

### Trilha Cascata - Comunidade Tradicional (Bairro da Cascata)

**Coordenação:** Isadora Nanci Ferreira (Bióloga + Anciã da Comunidade)
- [Site](https://ditaferreira.github.io/dita-website/)

| Ação | Descrição |
|------|-----------|
| Monitoramento | Fauna e flora local |
| Oficinas | Engajamento comunitário |
| Atividades Culturais | Integração com comunidade tradicional |
| Limpezas | Manutenção da trilha |
| Painel no App | Projeções holográficas de animais brasileiros |

**Importância Internacional:**
- Terras raras
- Água mineral radioativa (mais radioativa das Américas)
- Monitoramento popular especializado

---

## 6. PROVA DE CONCEITO

### Demo Atual
- 3 personagens + 3 locais de exemplo
- Funcional em Linux, Windows, VR e WebXR (parcial)

### Links
- **App AR:** https://technosutra.bhumisparshaschool.org/
- **Site:** https://technosutra84.wixstudio.com/stem-array
- **Tradução do Sutra:** https://84000.co/

### Assets Existentes
- 56 modelos 3D de Kalyanamitras (.glb)
- Textos dos 56 capítulos
- Código base Vortex R3D

---

## 7. ENTREGÁVEIS

| Item | Descrição |
|------|-----------|
| App VR completo | 56 capítulos + 112 pontos panorâmicos |
| Cross-device | Linux, Windows, VR, WebXR, iOS, Android |
| HUD completo | Menus, Save/Load, Config |
| PDF Reader | Leitura integrada de livros/documentos |
| Narração | Áudio para cada personagem |
| Código Open Source | Licenciado e documentado para jovens artistas/programadores |
| CI/CD | Build automatizado via GitHub Actions |

---

## 8. ORÇAMENTO RESUMIDO

| Categoria | Valor (R$) |
|-----------|------------|
| Diretoria (4 diretores) | 100.000,00 |
| Produção de Conteúdo | 15.000,00 |
| Equipamentos | 32.000,00 |
| Infraestrutura Digital | 3.000,00 |
| Contrapartida Ecológica | 25.000,00 |
| Marketing | 15.000,00 |
| Contingência | 10.000,00 |
| **TOTAL** | **200.000,00** |

---

## 9. IMPACTO

- **Cultural:** Democratização de texto budista milenar
- **Tecnológico:** Código open source para comunidade brasileira de gamedev
- **Ambiental:** Monitoramento e preservação da Trilha Cascata
- **Social:** Engajamento com comunidade tradicional
- **Internacional:** Conexão SP ↔ Índia/Nepal/Tibet

---

*Proposta Técnica - TECHNO SUTRA VR*
*Versão 2.0 - Janeiro 2026*
