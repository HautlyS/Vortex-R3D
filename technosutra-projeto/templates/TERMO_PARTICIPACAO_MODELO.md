<html>
<head>
<style>
@import url('https://fonts.googleapis.com/css2?family=Playfair+Display:wght@400;600;700&family=Inter:wght@300;400;500;600&display=swap');

:root {
  --gold: #C9A227;
  --gold-light: #E8D48A;
  --deep-purple: #2D1B4E;
  --soft-purple: #4A3366;
  --cream: #FDF8F0;
  --text: #2C2C2C;
  --text-light: #666;
}

* { box-sizing: border-box; margin: 0; padding: 0; }

body {
  font-family: 'Inter', -apple-system, sans-serif;
  background: var(--cream);
  color: var(--text);
  line-height: 1.7;
  min-height: 100vh;
  position: relative;
}

/* Textura de papel */
body::before {
  content: '';
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background-image: 
    url("data:image/svg+xml,%3Csvg viewBox='0 0 100 100' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)' opacity='0.04'/%3E%3C/svg%3E");
  pointer-events: none;
  z-index: 0;
}

/* Marca d'água */
body::after {
  content: '☸ TECHNO SUTRA VR ☸';
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%) rotate(-30deg);
  font-family: 'Playfair Display', serif;
  font-size: 6rem;
  color: var(--gold);
  opacity: 0.04;
  white-space: nowrap;
  pointer-events: none;
  z-index: 0;
  letter-spacing: 0.5rem;
}

.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 40px;
  position: relative;
  z-index: 1;
}

/* Header */
.header {
  background: linear-gradient(135deg, var(--deep-purple) 0%, var(--soft-purple) 100%);
  border-radius: 20px 20px 0 0;
  padding: 40px;
  text-align: center;
  position: relative;
  overflow: hidden;
  box-shadow: 0 10px 40px rgba(45, 27, 78, 0.3);
}

.header::before {
  content: '';
  position: absolute;
  top: -50%; left: -50%;
  width: 200%; height: 200%;
  background: radial-gradient(circle, rgba(201, 162, 39, 0.1) 0%, transparent 50%);
  animation: pulse 8s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { transform: scale(1); opacity: 0.5; }
  50% { transform: scale(1.1); opacity: 0.8; }
}

.header-icon {
  font-size: 3rem;
  margin-bottom: 15px;
  display: block;
}

.header h1 {
  font-family: 'Playfair Display', serif;
  font-size: 2.2rem;
  color: white;
  margin-bottom: 10px;
  position: relative;
  text-shadow: 0 2px 10px rgba(0,0,0,0.3);
}

.header .subtitle {
  color: var(--gold-light);
  font-size: 0.95rem;
  font-weight: 400;
  letter-spacing: 2px;
  text-transform: uppercase;
}

.header .project-badge {
  display: inline-block;
  background: var(--gold);
  color: var(--deep-purple);
  padding: 8px 20px;
  border-radius: 30px;
  font-weight: 600;
  font-size: 0.85rem;
  margin-top: 15px;
  box-shadow: 0 4px 15px rgba(201, 162, 39, 0.4);
}

/* Main content */
.content {
  background: white;
  padding: 50px;
  border-radius: 0 0 20px 20px;
  box-shadow: 0 20px 60px rgba(0,0,0,0.08);
  border: 1px solid rgba(201, 162, 39, 0.2);
  border-top: none;
}

/* Section styling */
.section {
  margin-bottom: 40px;
  padding-bottom: 30px;
  border-bottom: 1px solid rgba(201, 162, 39, 0.2);
}

.section:last-child {
  border-bottom: none;
  margin-bottom: 0;
}

.section-title {
  font-family: 'Playfair Display', serif;
  font-size: 1.4rem;
  color: var(--deep-purple);
  margin-bottom: 25px;
  display: flex;
  align-items: center;
  gap: 12px;
}

.section-title::before {
  content: '';
  width: 4px;
  height: 28px;
  background: linear-gradient(180deg, var(--gold) 0%, var(--gold-light) 100%);
  border-radius: 2px;
}

/* Form fields */
.form-grid {
  display: grid;
  gap: 20px;
}

.form-row {
  display: grid;
  grid-template-columns: 180px 1fr;
  align-items: center;
  gap: 15px;
}

.form-label {
  font-weight: 500;
  color: var(--text);
  font-size: 0.9rem;
}

.form-field {
  background: linear-gradient(to bottom, #FAFAFA, #F5F5F5);
  border: 1px solid #E0E0E0;
  border-radius: 10px;
  padding: 14px 18px;
  min-height: 48px;
  position: relative;
  transition: all 0.3s ease;
}

.form-field:hover {
  border-color: var(--gold);
  box-shadow: 0 0 0 3px rgba(201, 162, 39, 0.1);
}

.form-field::after {
  content: '✎';
  position: absolute;
  right: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: #CCC;
  font-size: 0.8rem;
}

/* Checkbox items */
.checkbox-item {
  display: flex;
  align-items: flex-start;
  gap: 15px;
  padding: 18px 20px;
  background: linear-gradient(135deg, #FAFAFA 0%, #F8F6F0 100%);
  border-radius: 12px;
  margin-bottom: 15px;
  border: 1px solid transparent;
  transition: all 0.3s ease;
}

.checkbox-item:hover {
  border-color: var(--gold-light);
  transform: translateX(5px);
}

.checkbox-box {
  width: 24px;
  height: 24px;
  border: 2px solid var(--gold);
  border-radius: 6px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: white;
}

.checkbox-text {
  font-size: 0.95rem;
  line-height: 1.6;
  color: var(--text);
}

.checkbox-text strong {
  color: var(--deep-purple);
}

/* Signature area */
.signature-area {
  background: linear-gradient(135deg, #F8F6F0 0%, #FDF8F0 100%);
  border: 2px dashed var(--gold-light);
  border-radius: 15px;
  padding: 40px;
  text-align: center;
  margin-top: 30px;
}

.signature-line {
  border-bottom: 2px solid var(--text);
  width: 70%;
  margin: 30px auto 10px;
  padding-bottom: 5px;
}

.signature-label {
  color: var(--text-light);
  font-size: 0.85rem;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.date-location {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 30px;
  margin-bottom: 30px;
}

/* Footer */
.footer {
  margin-top: 40px;
  padding-top: 30px;
  border-top: 2px solid var(--gold-light);
  text-align: center;
}

.footer-text {
  color: var(--text-light);
  font-size: 0.8rem;
  line-height: 1.8;
}

.footer-logo {
  font-family: 'Playfair Display', serif;
  color: var(--gold);
  font-size: 1.1rem;
  margin-bottom: 10px;
}

/* Decorative elements */
.divider {
  display: flex;
  align-items: center;
  gap: 15px;
  margin: 30px 0;
  color: var(--gold);
}

.divider::before,
.divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--gold-light), transparent);
}

.divider-icon {
  font-size: 1.2rem;
}

/* Witnesses table */
.witnesses-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 30px;
}

.witness-card {
  background: #FAFAFA;
  border-radius: 12px;
  padding: 25px;
  border: 1px solid #E8E8E8;
}

.witness-title {
  font-weight: 600;
  color: var(--deep-purple);
  margin-bottom: 15px;
  font-size: 0.9rem;
}

@media print {
  body { background: white; }
  body::before, body::after { display: none; }
  .container { padding: 20px; }
  .header { box-shadow: none; }
  .content { box-shadow: none; }
}
</style>
</head>
<body>
<div class="container">

<div class="header">
  <span class="header-icon">📜</span>
  <h1>TERMO DE PARTICIPAÇÃO</h1>
  <div class="subtitle">Edital FOMENTO CULTSP - PNAB Nº 12/2025</div>
  <div class="project-badge">☸ TECHNO SUTRA VR</div>
</div>

<div class="content">

<div class="section">
  <h2 class="section-title">Identificação do Integrante</h2>
  <div class="form-grid">
    <div class="form-row">
      <span class="form-label">Nome Completo</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">CPF</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">RG</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">Data de Nascimento</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">Endereço</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">Cidade/UF</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">CEP</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">Telefone</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">E-mail</span>
      <div class="form-field"></div>
    </div>
  </div>
</div>

<div class="section">
  <h2 class="section-title">Função no Projeto</h2>
  <div class="form-grid">
    <div class="form-row">
      <span class="form-label">Função</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">Período</span>
      <div class="form-field">Mês ___ a Mês ___</div>
    </div>
    <div class="form-row">
      <span class="form-label">Dedicação</span>
      <div class="form-field">____%</div>
    </div>
    <div class="form-row">
      <span class="form-label">Remuneração</span>
      <div class="form-field">R$ </div>
    </div>
  </div>
</div>

<div class="divider"><span class="divider-icon">☸</span></div>

<div class="section">
  <h2 class="section-title">Declarações</h2>
  
  <p style="color: var(--text-light); margin-bottom: 20px; font-size: 0.9rem;">
    Eu, acima identificado(a), <strong>DECLARO</strong> para os devidos fins que:
  </p>

  <div class="checkbox-item">
    <div class="checkbox-box"></div>
    <div class="checkbox-text">
      <strong>1. Participação no Projeto</strong><br>
      Concordo em participar do projeto TECHNO SUTRA VR na função acima especificada, comprometendo-me a cumprir as atividades e prazos estabelecidos.
    </div>
  </div>

  <div class="checkbox-item">
    <div class="checkbox-box"></div>
    <div class="checkbox-text">
      <strong>2. Veracidade das Informações</strong><br>
      Todas as informações prestadas neste termo são verdadeiras e podem ser comprovadas mediante solicitação.
    </div>
  </div>

  <div class="checkbox-item">
    <div class="checkbox-box"></div>
    <div class="checkbox-text">
      <strong>3. Direitos Autorais</strong><br>
      Autorizo o uso de minha imagem, voz e trabalho produzido no âmbito do projeto para fins de divulgação, documentação e prestação de contas.
    </div>
  </div>

  <div class="checkbox-item">
    <div class="checkbox-box"></div>
    <div class="checkbox-text">
      <strong>4. Exclusividade</strong><br>
      Declaro que não possuo vínculo com outro projeto contemplado neste mesmo edital que possa configurar conflito de interesses.
    </div>
  </div>

  <div class="checkbox-item">
    <div class="checkbox-box"></div>
    <div class="checkbox-text">
      <strong>5. Ciência do Edital</strong><br>
      Declaro ter conhecimento das regras do Edital FOMENTO CULTSP - PNAB Nº 12/2025 e me comprometo a cumpri-las.
    </div>
  </div>
</div>

<div class="section">
  <h2 class="section-title">Dados Bancários</h2>
  <div class="form-grid">
    <div class="form-row">
      <span class="form-label">Banco</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">Agência</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">Conta</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">Tipo de Conta</span>
      <div class="form-field">☐ Corrente &nbsp;&nbsp; ☐ Poupança</div>
    </div>
    <div class="form-row">
      <span class="form-label">Titular</span>
      <div class="form-field"></div>
    </div>
    <div class="form-row">
      <span class="form-label">CPF do Titular</span>
      <div class="form-field"></div>
    </div>
  </div>
</div>

<div class="divider"><span class="divider-icon">✦</span></div>

<div class="signature-area">
  <p style="font-style: italic; color: var(--text-light); margin-bottom: 20px;">
    Por ser verdade, firmo o presente termo.
  </p>
  
  <div class="date-location">
    <div>
      <div class="form-field" style="margin-bottom: 8px;"></div>
      <span class="signature-label">Local</span>
    </div>
    <div>
      <div class="form-field" style="margin-bottom: 8px;">___/___/2026</div>
      <span class="signature-label">Data</span>
    </div>
  </div>
  
  <div class="signature-line"></div>
  <span class="signature-label">Assinatura do Integrante</span>
</div>

<div class="section" style="margin-top: 40px;">
  <h2 class="section-title">Testemunhas (opcional)</h2>
  <div class="witnesses-grid">
    <div class="witness-card">
      <div class="witness-title">Testemunha 1</div>
      <div class="form-grid">
        <div class="form-row" style="grid-template-columns: 60px 1fr;">
          <span class="form-label">Nome</span>
          <div class="form-field" style="min-height: 36px;"></div>
        </div>
        <div class="form-row" style="grid-template-columns: 60px 1fr;">
          <span class="form-label">CPF</span>
          <div class="form-field" style="min-height: 36px;"></div>
        </div>
      </div>
      <div style="border-bottom: 1px solid #CCC; margin-top: 20px; padding-bottom: 5px;"></div>
      <span class="signature-label" style="font-size: 0.75rem;">Assinatura</span>
    </div>
    <div class="witness-card">
      <div class="witness-title">Testemunha 2</div>
      <div class="form-grid">
        <div class="form-row" style="grid-template-columns: 60px 1fr;">
          <span class="form-label">Nome</span>
          <div class="form-field" style="min-height: 36px;"></div>
        </div>
        <div class="form-row" style="grid-template-columns: 60px 1fr;">
          <span class="form-label">CPF</span>
          <div class="form-field" style="min-height: 36px;"></div>
        </div>
      </div>
      <div style="border-bottom: 1px solid #CCC; margin-top: 20px; padding-bottom: 5px;"></div>
      <span class="signature-label" style="font-size: 0.75rem;">Assinatura</span>
    </div>
  </div>
</div>

<div class="footer">
  <div class="footer-logo">☸ TECHNO SUTRA VR ☸</div>
  <div class="footer-text">
    Jornada Imersiva do Avatamsaka Sutra<br>
    Edital FOMENTO CULTSP - PNAB Nº 12/2025 • Módulo III - VR/MR<br>
    <em>Este documento deve ser assinado por todos os integrantes do projeto</em>
  </div>
</div>

</div>
</div>
</body>
</html>
