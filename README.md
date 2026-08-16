# Ortografic — Verificador Ortográfico

Aplicação desktop leve e rápida para verificação ortográfica e sugestão de palavras. O projeto utiliza uma estrutura **BK-Tree** combinada com o algoritmo de **Distância de Levenshtein** implementados em Rust para buscas eficientes.

## 🚀 Tecnologias Utilizadas

- **Frontend:** React, TypeScript, Vite, Tailwind CSS
- **Backend:** Rust, Tauri
- **Algoritmo:** BK-Tree (Burkhard-Keller Tree) + Levenshtein Distance

## 📌 Funcionalidades

- Verificação ortográfica de texto em tempo real.
- Sugestão de palavras semelhantes com base na distância de edição (Levenshtein).
- Consulta rápida utilizando dicionário local otimizado.

## 🛠️ Como Executar o Projeto

### Pré-requisitos
- [Node.js](https://nodejs.org/) instalado.
- [Rust](https://www.rust-lang.org/) instalado.

### Passo a Passo

1. **Clone o repositório:**
```bash
    git clone [https://github.com/seu-usuario/seu-repositorio.git](https://github.com/seu-usuario/seu-repositorio.git)
    cd seu-repositorio
```
2. Instale as dependências do Frontend:

```bash
    npm install
```

3. Inicie o aplicativo em modo de desenvolvimento:

```bash
    npm run tauri dev
```


## 📦 Como Gerar o Executável (Build)
Para compilar a aplicação e gerar o instalador do sistema operacional:

```bash
    npm run tauri build
```
💡 O arquivo de instalação (.msi / .exe no Windows ou .dmg / .app no macOS) será gerado na pasta:
src-tauri/target/release/bundle/

