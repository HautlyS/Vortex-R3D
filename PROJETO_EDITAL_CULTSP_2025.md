# 🕉️ TECHNO SUTRA: VIRTUAL WISDOM
## Proposta para Edital FOMENTO CULTSP - PNAB Nº 12/2025
### Módulo III - Realidade Virtual e Mista (VR/MR) - R$ 200.000,00

---

## 1. IDENTIFICAÇÃO DO PROJETO

**Nome:** Techno Sutra: Virtual Wisdom  
**Categoria:** Módulo III - VR/MR (Realidade Virtual e Mista)  
**Valor Solicitado:** R$ 200.000,00  
**Prazo de Execução:** 12 meses  

---

## 2. SINOPSE

**Techno Sutra: Virtual Wisdom** é uma experiência imersiva em Realidade Virtual que transporta o usuário para uma jornada espiritual através dos 56 capítulos do Avatamsaka Sutra (Sutra Gandavyuha), um dos textos mais importantes do budismo Mahayana. O projeto estabelece uma **ponte cultural inédita entre São Paulo e Katmandu**, unindo a cena artística futurista paulistana com a tradição milenar budista do Nepal.

A experiência permite que o usuário siga os passos de Sudhana, o protagonista do sutra, através de **salas imersivas interconectadas por portais**, cada uma representando um capítulo e seu respectivo mestre espiritual. Modelos 3D de alta qualidade, trilha sonora original de bandas do cenário SP Futurista, e narrativa interativa criam uma experiência única de contemplação e aprendizado.

---

## 3. JUSTIFICATIVA E RELEVÂNCIA CULTURAL

### 3.1 Conexão São Paulo - Katmandu

O projeto representa uma **colaboração cultural internacional** entre:
- **São Paulo**: Direção artística, trilha sonora (bandas do cenário SP Futurista), design gráfico e desenvolvimento tecnológico
- **Katmandu**: Consultoria espiritual, autenticidade dos ensinamentos, conexão com tradições budistas vivas

Esta parceria democratiza o acesso a ensinamentos budistas autênticos, tradicionalmente restritos a monastérios e centros especializados, tornando-os acessíveis através da tecnologia imersiva.

### 3.2 Preservação e Difusão Cultural

O Avatamsaka Sutra é um texto de **importância histórica e filosófica mundial**, influenciando:
- Filosofia oriental e ocidental
- Arte e arquitetura asiática
- Práticas contemplativas contemporâneas

O projeto digitaliza e preserva este patrimônio cultural imaterial, criando um **arquivo vivo e interativo** para gerações futuras.

### 3.3 Inovação Tecnológica Brasileira

O desenvolvimento utiliza **tecnologias 100% open source**, contribuindo para:
- Capacitação técnica da comunidade brasileira de desenvolvedores
- Independência tecnológica nacional
- Modelo replicável para outros projetos culturais

---

## 4. DIFERENCIAIS TECNOLÓGICOS

### 4.1 Rust + Bevy Engine: Escolha Estratégica

O projeto utiliza **Rust** como linguagem principal e **Bevy** como engine de jogos, uma escolha técnica que oferece vantagens significativas:

| Aspecto | Rust/Bevy | Unity/Unreal | Godot |
|---------|-----------|--------------|-------|
| **Performance** | Nativa, sem GC | Garbage Collection | GDScript lento |
| **Segurança de Memória** | Garantida em compilação | Runtime errors | Runtime errors |
| **Tamanho do Build** | ~15MB (WASM) | ~100MB+ | ~50MB+ |
| **Licenciamento** | MIT/Apache (100% livre) | Royalties/Subscrição | MIT |
| **Cross-platform** | Nativo | Camada de abstração | Camada de abstração |
| **WebXR** | Suporte nativo | Plugin pago | Experimental |
| **Contribuição Open Source** | Direto no ecossistema | Fechado | Limitado |

#### Por que Rust?

1. **Segurança sem Sacrifício**: Rust elimina classes inteiras de bugs (null pointers, data races) em tempo de compilação, crucial para experiências VR onde crashes causam desconforto físico.

2. **Performance Previsível**: Sem garbage collector, a experiência mantém 90fps constantes, essencial para evitar motion sickness em VR.

3. **Ecossistema Crescente**: Rust é a linguagem mais amada por desenvolvedores por 8 anos consecutivos (Stack Overflow Survey), garantindo longevidade do projeto.

4. **WebAssembly Nativo**: Compilação direta para WASM permite execução em navegadores sem plugins, democratizando o acesso.

### 4.2 Arquitetura Multi-Plataforma

```
┌─────────────────────────────────────────────────────────────┐
│                    TECHNO SUTRA ENGINE                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Desktop   │  │     VR      │  │   WebXR     │         │
│  │  (Windows/  │  │  (Quest/    │  │  (Browser)  │         │
│  │  Linux/Mac) │  │  SteamVR)   │  │             │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
│         │                │                │                 │
│         └────────────────┼────────────────┘                 │
│                          │                                  │
│              ┌───────────┴───────────┐                      │
│              │     Bevy 0.17 Core    │                      │
│              │  (ECS Architecture)   │                      │
│              └───────────┬───────────┘                      │
│                          │                                  │
│  ┌───────────────────────┼───────────────────────┐         │
│  │                       │                       │         │
│  ▼                       ▼                       ▼         │
│ Panorama    Portals    World    BookReader    Audio        │
│ Plugin      Plugin     Plugin   Plugin        Plugin       │
└─────────────────────────────────────────────────────────────┘
```

### 4.3 Features Técnicas Implementadas

| Feature | Descrição | Status |
|---------|-----------|--------|
| **Panoramas 360°** | Conversão equirectangular→cubemap em GPU | ✅ Implementado |
| **Sistema de Portais** | Render-to-texture com efeito líquido WGSL | ✅ Implementado |
| **Salas Interconectadas** | 56 ambientes únicos com transição suave | ✅ Implementado |
| **Modelos GLB/GLTF** | 56 personagens 3D otimizados | ✅ Implementado |
| **Áudio Espacial** | Trilha sonora posicional 3D | ✅ Implementado |
| **Book Reader** | Leitura imersiva dos capítulos | ✅ Implementado |
| **VR Nativo** | OpenXR via bevy_oxr | ✅ Implementado |
| **WebXR** | Acesso via navegador | 🔄 Em desenvolvimento |
| **Partículas de Energia** | Sistema GPU com bevy_hanabi | ✅ Implementado |
| **Post-Processing** | Bloom, tonemapping, efeitos oníricos | ✅ Implementado |

### 4.4 Shaders WGSL Customizados

O projeto inclui shaders originais escritos em WGSL (WebGPU Shading Language):

- **portal_effect.wgsl**: Efeito de superfície líquida nos portais
- **dream_post_process.wgsl**: Atmosfera onírica e contemplativa
- **vortex_transition.wgsl**: Transições entre salas
- **spin_blur.wgsl**: Efeitos de movimento

---

## 5. CONTEÚDO E NARRATIVA

### 5.1 Os 56 Capítulos

Cada capítulo do Avatamsaka Sutra é representado por:
- **Sala imersiva única** com ambiente temático
- **Modelo 3D do mestre espiritual** (Bodhisattva, Deva, ou ser iluminado)
- **Texto do capítulo** acessível via Book Reader
- **Trilha sonora específica** composta por artistas paulistanos
- **Elementos interativos** relacionados aos ensinamentos

### 5.2 Jornada de Sudhana

O usuário assume o papel de Sudhana, jovem buscador que visita 53 mestres espirituais em sua jornada para a iluminação. A narrativa é não-linear, permitindo:
- Exploração livre entre salas
- Retorno a mestres anteriores
- Descoberta de conexões entre ensinamentos

### 5.3 Personagens Principais

| Mestre | Capítulo | Ensinamento |
|--------|----------|-------------|
| Manjushri | 3 | Sabedoria primordial |
| Samantabhadra | 2, 56 | Ação compassiva |
| Avalokiteshvara | 30 | Compaixão universal |
| Maitreya | 54 | Futuro Buddha |
| Vasumitra | 28 | Amor como caminho |

---

## 6. TRILHA SONORA: SP FUTURISTA

### 6.1 Parceria Musical

A trilha sonora será composta por artistas do cenário **SP Futurista**, movimento que une:
- Música eletrônica experimental
- Influências da cultura brasileira
- Estética cyberpunk e afrofuturista

### 6.2 Direção Sonora

- **Composições originais** para cada sala/capítulo
- **Áudio espacial 3D** integrado à engine
- **Licenciamento Creative Commons** para difusão

---

## 7. PLANO DE DEMOCRATIZAÇÃO

### 7.1 Acesso Universal

| Plataforma | Requisito | Custo para Usuário |
|------------|-----------|-------------------|
| **Web (WebXR)** | Navegador moderno | Gratuito |
| **Desktop** | PC básico | Gratuito |
| **VR Standalone** | Meta Quest 2/3 | Gratuito |
| **VR PC** | SteamVR | Gratuito |

### 7.2 Ações Formativas

1. **Making-of em Vídeo**: Série documentando o desenvolvimento
2. **Tutoriais Técnicos**: Rust, Bevy, VR development
3. **Workshops em Escolas Públicas**: Introdução a XR e programação
4. **Palestras em Universidades**: Tecnologia e cultura

### 7.3 Código Aberto

**100% do código será disponibilizado em repositórios públicos:**
- GitHub: https://github.com/[projeto]
- Codeberg: https://codeberg.org/[projeto]

Licença: **MIT/Apache 2.0** (dual license)

---

## 8. PLANO DE ACESSIBILIDADE

### 8.1 Acessibilidade Visual

- **Alto contraste** configurável
- **Legendas** para todo conteúdo de áudio
- **Descrição de áudio** para elementos visuais
- **Tamanho de fonte** ajustável

### 8.2 Acessibilidade Auditiva

- **Legendas completas** em português e inglês
- **Indicadores visuais** para eventos sonoros
- **Vibração háptica** (em dispositivos compatíveis)

### 8.3 Acessibilidade Motora

- **Controles simplificados** (one-handed mode)
- **Teleporte** como alternativa a locomoção contínua
- **Tempo de interação** configurável
- **Modo sentado** para VR

### 8.4 Acessibilidade Cognitiva

- **Navegação simplificada** opcional
- **Resumos** dos capítulos
- **Modo guiado** com narração

---

## 9. CRONOGRAMA DE EXECUÇÃO

| Mês | Atividade | Entregável |
|-----|-----------|------------|
| 1-2 | Pré-produção | Design document, storyboard completo |
| 3-4 | Produção de Assets | 56 salas modeladas, texturas |
| 5-6 | Desenvolvimento Core | Engine features, portais, navegação |
| 7-8 | Integração de Conteúdo | Modelos, áudio, textos |
| 9-10 | Trilha Sonora | Composições, mixagem, integração |
| 11 | Testes e Otimização | QA, performance, acessibilidade |
| 12 | Lançamento | Deploy, documentação, divulgação |

---

## 10. ORÇAMENTO DETALHADO

| Item | Descrição | Valor (R$) |
|------|-----------|------------|
| **Desenvolvimento** | | |
| Programação Rust/Bevy | 6 meses, 2 desenvolvedores | 72.000,00 |
| Shaders e Efeitos Visuais | Especialista WGSL | 12.000,00 |
| **Arte e Design** | | |
| Direção de Arte | Conceito visual, supervisão | 18.000,00 |
| Modelagem 3D | Refinamento dos 56 modelos | 24.000,00 |
| Ambientação | Design das 56 salas | 15.000,00 |
| **Áudio** | | |
| Trilha Sonora | Composição, produção | 20.000,00 |
| Sound Design | Efeitos, ambientação | 8.000,00 |
| **Produção** | | |
| Gestão de Projeto | Coordenação, cronograma | 12.000,00 |
| Equipamentos | Hardware VR para testes | 8.000,00 |
| **Democratização** | | |
| Workshops e Palestras | 6 eventos | 6.000,00 |
| Documentação e Tutoriais | Vídeos, textos | 5.000,00 |
| **TOTAL** | | **200.000,00** |

---

## 11. EQUIPE TÉCNICA

### 11.1 Núcleo Principal

| Função | Responsabilidade |
|--------|------------------|
| **Direção Geral** | Visão artística, coordenação |
| **Lead Developer** | Arquitetura Rust/Bevy, VR |
| **3D Artist** | Modelagem, texturização |
| **Sound Designer** | Trilha, áudio espacial |
| **UX Designer** | Interface, acessibilidade |

### 11.2 Colaboradores

- **Consultoria Budista**: Autenticidade dos ensinamentos
- **Bandas SP Futurista**: Trilha sonora original
- **Comunidade Open Source**: Contribuições de código

---

## 12. POTENCIAL DE IMPACTO

### 12.1 Impacto Cultural

- **Preservação digital** de patrimônio imaterial
- **Ponte intercultural** Brasil-Nepal
- **Democratização** de ensinamentos tradicionais

### 12.2 Impacto Tecnológico

- **Referência técnica** para projetos XR em Rust
- **Capacitação** da comunidade brasileira
- **Modelo open source** replicável

### 12.3 Impacto Educacional

- **Material didático** para escolas e universidades
- **Introdução à programação** via projeto real
- **Discussão sobre tecnologia e espiritualidade**

### 12.4 Alcance Estimado

| Métrica | Ano 1 | Ano 2 |
|---------|-------|-------|
| Downloads/Acessos | 10.000 | 50.000 |
| Workshops realizados | 6 | 12 |
| Contribuidores open source | 20 | 100 |
| Países alcançados | 10 | 30 |

---

## 13. SUSTENTABILIDADE

### 13.1 Pós-Projeto

- **Manutenção comunitária** via open source
- **Atualizações** com novas features
- **Expansões** para outros sutras/tradições

### 13.2 Modelo de Continuidade

- Doações voluntárias (Ko-fi, GitHub Sponsors)
- Parcerias com instituições educacionais
- Licenciamento para museus e centros culturais

---

## 14. LINKS E REFERÊNCIAS

### 14.1 Repositórios

- **Código Fonte**: [GitHub/Codeberg - a ser publicado]
- **Assets Legacy**: technosutra/ (56 modelos GLB/USDZ)
- **Documentação**: docs/

### 14.2 Tecnologias Utilizadas

- **Rust**: https://www.rust-lang.org/
- **Bevy Engine**: https://bevyengine.org/
- **OpenXR**: https://www.khronos.org/openxr/
- **WebGPU/WGSL**: https://www.w3.org/TR/webgpu/

### 14.3 Referências Culturais

- **Avatamsaka Sutra**: Tradução Thomas Cleary
- **Khyentse Foundation**: Apoio a projetos dhármicos
- **84000**: Tradução de textos budistas

---

## 15. DECLARAÇÃO DE ORIGINALIDADE

Declaro que este projeto é uma obra original, desenvolvida com tecnologias open source, e que todos os direitos autorais dos conteúdos utilizados estão devidamente licenciados ou são de domínio público.

O código-fonte será integralmente disponibilizado sob licença MIT/Apache 2.0, permitindo uso, modificação e distribuição livre por qualquer pessoa ou instituição.

---

## 16. CONCLUSÃO

**Techno Sutra: Virtual Wisdom** representa uma convergência única entre:
- **Tradição milenar** e **tecnologia de ponta**
- **Cultura brasileira** e **sabedoria oriental**
- **Arte imersiva** e **código aberto**

O projeto não apenas cria uma experiência cultural inovadora, mas estabelece um **modelo técnico e metodológico** que pode ser replicado por outros projetos culturais brasileiros, fortalecendo o ecossistema de desenvolvimento XR nacional.

A escolha por Rust e tecnologias open source garante que o investimento público retorne à sociedade na forma de **conhecimento compartilhado**, **capacitação técnica** e **infraestrutura reutilizável**.

---

*"Assim como uma gota de água contém o oceano inteiro, cada momento de contemplação contém a totalidade da sabedoria."*
— Avatamsaka Sutra

---

**Techno Sutra: Virtual Wisdom**  
*Uma jornada imersiva do código ao despertar*

🕉️ São Paulo ↔ Katmandu 🏔️
