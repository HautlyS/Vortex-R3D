# PLANO DE ACESSIBILIDADE
## TECHNO SUTRA VR
### Edital FOMENTO CULTSP - PNAB Nº 12/2025

---

## 1. INTRODUÇÃO

O Plano de Acessibilidade do projeto TECHNO SUTRA VR estabelece as medidas comunicacionais, arquitetônicas e atitudinais que serão implementadas para garantir que a experiência seja acessível ao maior número possível de pessoas, incluindo pessoas com deficiência.

### 1.1 Princípios Norteadores
- **Universalidade**: Design para todos desde o início
- **Flexibilidade**: Múltiplas formas de interação
- **Simplicidade**: Interface intuitiva e clara
- **Perceptibilidade**: Informação disponível em múltiplos formatos
- **Tolerância ao erro**: Prevenção e recuperação de erros

### 1.2 Referências
- WCAG 2.1 (Web Content Accessibility Guidelines)
- XR Accessibility User Requirements (W3C)
- Meta Quest Accessibility Guidelines
- Lei Brasileira de Inclusão (Lei 13.146/2015)

---

## 2. ACESSIBILIDADE VISUAL

### 2.1 Deficiência Visual Total

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Audiodescrição** | Narração detalhada de ambientes e personagens | Alta |
| **Navegação por áudio** | Indicadores sonoros de direção e interação | Alta |
| **Modo somente áudio** | Experiência completa sem dependência visual | Média |
| **Feedback háptico** | Vibração dos controles para indicar eventos | Alta |

### 2.2 Baixa Visão

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Alto contraste** | Opção de cores de alto contraste na UI | Alta |
| **Tamanho de fonte** | Ajuste de 100% a 200% | Alta |
| **Zoom de UI** | Ampliação de elementos de interface | Média |
| **Contornos destacados** | Bordas visíveis em elementos interativos | Alta |

### 2.3 Daltonismo

| Tipo | Solução |
|------|---------|
| **Protanopia** | Filtro de cores + símbolos adicionais |
| **Deuteranopia** | Filtro de cores + símbolos adicionais |
| **Tritanopia** | Filtro de cores + símbolos adicionais |
| **Acromático** | Modo monocromático com texturas distintas |

### 2.4 Configurações Visuais

```
┌─────────────────────────────────────┐
│     ⚙ ACESSIBILIDADE VISUAL        │
├─────────────────────────────────────┤
│  Tamanho da Fonte                   │
│  [====●====] 150%                   │
│                                     │
│  Contraste                          │
│  ( ) Normal                         │
│  (●) Alto Contraste                 │
│  ( ) Invertido                      │
│                                     │
│  Filtro de Daltonismo               │
│  [▼ Deuteranopia          ]         │
│                                     │
│  Audiodescrição                     │
│  [●] Ativada                        │
│                                     │
│         [Aplicar]  [Padrão]         │
└─────────────────────────────────────┘
```

---

## 3. ACESSIBILIDADE AUDITIVA

### 3.1 Deficiência Auditiva Total

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Legendas completas** | Todo áudio transcrito em texto | Alta |
| **Identificação de falante** | Nome do personagem nas legendas | Alta |
| **Descrição de sons** | [som de sino], [passos], etc. | Alta |
| **Indicadores visuais** | Ícones para eventos sonoros | Alta |

### 3.2 Baixa Audição

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Controle de volume** | Ajuste independente por categoria | Alta |
| **Amplificação de diálogos** | Boost de voz sobre ambiente | Média |
| **Equalização** | Ajuste de frequências | Baixa |

### 3.3 Formato de Legendas

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                    [Som de sino ao longe]                   │
│                                                             │
│  MANJUSHRI:                                                 │
│  "Suddhana, sua jornada começa agora.                       │
│   Vá para o sul e encontre Meghasri."                       │
│                                                             │
│                    [Música suave começa]                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 3.4 Configurações de Legendas

```
┌─────────────────────────────────────┐
│     ⚙ LEGENDAS E ÁUDIO             │
├─────────────────────────────────────┤
│  Legendas                           │
│  [●] Ativadas                       │
│                                     │
│  Tamanho das Legendas               │
│  [====●====] Grande                 │
│                                     │
│  Fundo das Legendas                 │
│  [▼ Preto semi-transparente ]       │
│                                     │
│  Descrição de Sons                  │
│  [●] Ativada                        │
│                                     │
│  Indicadores Visuais de Áudio       │
│  [●] Ativados                       │
│                                     │
│         [Aplicar]  [Padrão]         │
└─────────────────────────────────────┘
```

---

## 4. ACESSIBILIDADE MOTORA

### 4.1 Mobilidade Reduzida

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Modo sentado** | Experiência completa sentado | Alta |
| **Teleporte** | Movimento sem locomoção física | Alta |
| **Ajuste de altura** | Calibração de altura virtual | Alta |
| **Área de jogo reduzida** | Funcionamento em espaço mínimo | Média |

### 4.2 Dificuldade com Controles

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Controle por olhar (gaze)** | Seleção por direção do olhar | Alta |
| **Um único botão** | Todas as ações com um botão | Alta |
| **Tempo de seleção ajustável** | 0.5s a 3s para confirmar | Alta |
| **Remapeamento de controles** | Personalização completa | Média |
| **Modo automático** | Progressão sem interação | Média |

### 4.3 Tremores/Precisão

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Áreas de toque ampliadas** | Botões maiores | Alta |
| **Estabilização de mira** | Suavização de movimento | Média |
| **Confirmação de ação** | Evitar ações acidentais | Alta |
| **Desfazer ação** | Reverter última ação | Média |

### 4.4 Configurações de Controle

```
┌─────────────────────────────────────┐
│     ⚙ CONTROLES E MOVIMENTO        │
├─────────────────────────────────────┤
│  Modo de Interação                  │
│  ( ) Controles padrão               │
│  (●) Controle por olhar             │
│  ( ) Um único botão                 │
│                                     │
│  Tempo de Seleção                   │
│  [====●====] 1.5 segundos           │
│                                     │
│  Modo de Movimento                  │
│  (●) Teleporte                      │
│  ( ) Locomoção suave                │
│                                     │
│  Posição                            │
│  (●) Sentado                        │
│  ( ) Em pé                          │
│                                     │
│  Estabilização de Mira              │
│  [●] Ativada                        │
│                                     │
│         [Aplicar]  [Padrão]         │
└─────────────────────────────────────┘
```

---

## 5. ACESSIBILIDADE COGNITIVA

### 5.1 Dificuldades de Aprendizagem

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Tutorial interativo** | Passo a passo com prática | Alta |
| **Dicas contextuais** | Ajuda quando necessário | Alta |
| **Linguagem simples** | Textos claros e diretos | Alta |
| **Repetição de instruções** | Opção de reouvir | Média |

### 5.2 Dificuldades de Memória

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Resumo de progresso** | O que aconteceu até agora | Alta |
| **Objetivos visíveis** | Sempre mostrar próximo passo | Alta |
| **Diário de jornada** | Registro de encontros | Média |
| **Mapa de progresso** | Visualização da jornada | Média |

### 5.3 TDAH/Dificuldade de Foco

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Modo foco** | Redução de distrações visuais | Média |
| **Pausas sugeridas** | Lembretes de descanso | Baixa |
| **Sessões curtas** | Capítulos de 5-10 minutos | Alta |
| **Salvamento frequente** | Autosave a cada interação | Alta |

---

## 6. CONFORTO EM VR

### 6.1 Prevenção de Enjoo (Motion Sickness)

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Vinheta de conforto** | Escurecimento periférico em movimento | Alta |
| **Teleporte snap** | Rotação em incrementos (30°, 45°, 90°) | Alta |
| **Ponto de referência fixo** | Elemento estático no campo de visão | Média |
| **Velocidade ajustável** | Controle de velocidade de movimento | Alta |

### 6.2 Fadiga Visual

| Recurso | Implementação | Prioridade |
|---------|---------------|------------|
| **Lembretes de pausa** | A cada 30 minutos | Média |
| **Ajuste de brilho** | Controle de intensidade | Alta |
| **Modo noturno** | Redução de luz azul | Baixa |
| **Distância de leitura** | Textos a distância confortável | Alta |

### 6.3 Configurações de Conforto

```
┌─────────────────────────────────────┐
│     ⚙ CONFORTO VR                  │
├─────────────────────────────────────┤
│  Vinheta de Conforto                │
│  [====●====] Média                  │
│                                     │
│  Rotação Snap                       │
│  [▼ 45 graus              ]         │
│                                     │
│  Velocidade de Movimento            │
│  [==●======] Lenta                  │
│                                     │
│  Lembretes de Pausa                 │
│  [●] A cada 30 minutos              │
│                                     │
│  Brilho                             │
│  [======●==] 80%                    │
│                                     │
│         [Aplicar]  [Padrão]         │
└─────────────────────────────────────┘
```

---

## 7. VERSÃO DESKTOP (NÃO-VR)

Para garantir acesso a pessoas que não podem usar VR, o projeto incluirá uma versão desktop completa:

### 7.1 Funcionalidades Desktop

| Recurso | Descrição |
|---------|-----------|
| **Experiência completa** | Todos os 56 capítulos |
| **Controles tradicionais** | Mouse + teclado ou gamepad |
| **Visualização 360°** | Arrastar para olhar ao redor |
| **Sem requisito de VR** | Funciona em qualquer PC |

### 7.2 Controles Desktop

| Entrada | Ação |
|---------|------|
| Mouse arrastar | Olhar ao redor |
| WASD / Setas | Olhar ao redor |
| Clique esquerdo | Interagir |
| Espaço | Avançar diálogo |
| Escape | Menu |
| Tab | Mapa de progresso |

---

## 8. TESTES DE ACESSIBILIDADE

### 8.1 Metodologia

| Fase | Atividade | Participantes |
|------|-----------|---------------|
| **Alpha** | Testes internos com checklist | Equipe |
| **Beta** | Testes com usuários diversos | 10 pessoas |
| **Release** | Validação final | 5 pessoas com deficiência |

### 8.2 Checklist de Testes

#### Visual
- [ ] Navegação completa sem visão (audiodescrição)
- [ ] Leitura de todos os textos em tamanho máximo
- [ ] Funcionamento com filtros de daltonismo
- [ ] Contraste suficiente em todos os elementos

#### Auditivo
- [ ] Compreensão completa com legendas
- [ ] Identificação de todos os eventos sonoros
- [ ] Funcionamento sem áudio

#### Motor
- [ ] Conclusão com controle por olhar
- [ ] Conclusão com um único botão
- [ ] Funcionamento sentado
- [ ] Funcionamento em espaço mínimo

#### Cognitivo
- [ ] Tutorial compreensível
- [ ] Objetivos sempre claros
- [ ] Possibilidade de repetir instruções

#### Conforto
- [ ] Sem enjoo com configurações de conforto
- [ ] Lembretes de pausa funcionando
- [ ] Brilho ajustável

### 8.3 Relatório de Acessibilidade

Será produzido um relatório documentando:
- Recursos implementados
- Resultados dos testes
- Feedback dos usuários
- Melhorias futuras planejadas

---

## 9. ORÇAMENTO DE ACESSIBILIDADE

| Item | Valor (R$) |
|------|------------|
| Desenvolvimento de recursos | Incluído no dev |
| Audiodescrição profissional | 2.000,00 |
| Testes com usuários | 1.000,00 |
| Consultoria de acessibilidade | 1.500,00 |
| **Total** | **4.500,00** |

*Nota: Valor incluído no orçamento geral do projeto*

---

## 10. COMPROMISSO

Declaro que me comprometo a implementar todas as medidas de acessibilidade descritas neste plano, garantindo que o projeto TECHNO SUTRA VR seja acessível ao maior número possível de pessoas, incluindo pessoas com deficiência visual, auditiva, motora e cognitiva.

**Local e Data**: _________________, ___/___/2026

**Assinatura do Proponente**: _________________________________

**Nome**: _________________________________

**CPF**: _________________________________

---

*Plano de Acessibilidade - TECHNO SUTRA VR*
*Versão 1.0 - Janeiro 2026*
