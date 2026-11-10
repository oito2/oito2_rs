# Documentação do oito2_rs

🌐 [English](../en/index.md) | **Português** | 🏠 [Voltar ao README](leiame.md)

O oito2_rs guarda as regras de plataforma compartilhadas pelo lado Rust do oito2: `oito2_foundation` (tipos e regras puros) e `oito2_settings` (leitores e observadores das configurações do sistema no Linux, Windows e macOS). Estas páginas explicam como usar os crates, como funcionam e como são construídos.

## 🚀 Primeiros passos

- [Instalação](getting-started/installation.md) — requisitos por sistema operacional, como adicionar os crates, como compilar a partir do código.
- [Início rápido](getting-started/quickstart.md) — ler, compor e observar as configurações em poucas linhas.

## 🧠 Conceitos

- [Por que o oito2_rs](concepts/why-oito2_rs.md) — o problema que os crates resolvem e para quem são.
- [Como o oito2_rs funciona](concepts/how-oito2_rs-works.md) — dos armazenamentos do sistema ao `SystemSettings`.
- [Arquitetura](concepts/architecture.md) — fluxo de dados, organização, dependências e decisões de projeto.
- [Glossário](concepts/glossary.md) — os termos usados nestas páginas.

## 🏗️ Arquitetura

- [Regras de composição](architecture/composition.md) — como o `compose` transforma valores brutos em configurações, por desktop.
- [Leitores](architecture/readers.md) — como os armazenamentos de cada plataforma são lidos.
- [Observadores](architecture/watchers.md) — threads, debounce, deduplicação e parada.
- [Paridade com o oito2_ui](architecture/parity.md) — a fixture, os testes e o workflow.

## 📖 Guias

- [Exemplos de integração](guides/workflows/examples.md) — um plugin Tauri, um canal para um loop de eventos, JSON para um webview.

## 📚 Referência

- [API](reference/api.md) — todo item público dos dois crates e suas features.
- [Contrato JSON](reference/json-contract.md) — o formato `serde` de cada tipo.
- [Fontes das configurações](reference/settings-sources.md) — cada chave, arquivo e API lidos, por plataforma.

## 🛠️ Solução de problemas

- [Problemas comuns](troubleshooting/common-issues.md) — erros de compilação, valores ausentes e mudanças não informadas.
