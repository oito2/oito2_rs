# Como contribuir com o oito2_rs

🌐 **Idioma:** [English](../../CONTRIBUTING.md) · Português

Obrigado pelo interesse no oito2_rs. Este guia cobre o ambiente de desenvolvimento, as regras que toda mudança segue e como as versões são publicadas.

## Onde as mudanças começam

O oito2_ui é a fonte da verdade de toda regra do `oito2_foundation`. Um desktop novo, uma configuração nova ou um fallback diferente mudam primeiro lá, chegam à fixture de paridade dele (`tools/parity/fixtures/foundation.json`) e só então são portados para cá. Bugs nos leitores, nos observadores e na API Rust são corrigidos diretamente neste repositório.

## Ambiente de desenvolvimento

- Rust pelo `rustup`: o `rust-toolchain.toml` fixa a 1.99.0 com `rustfmt` e `clippy`. A MSRV é 1.90.
- Linux: `libglib2.0-dev` e `pkg-config` (ou o equivalente da sua distribuição), mais o `dbus` para o `dbus-run-session`.
- [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) para a verificação de licenças e advisories.

```bash
git clone https://github.com/oito2/oito2_rs.git
cd oito2_rs
rustup toolchain install 1.90 --profile minimal
cargo install cargo-deny --locked
```

## Organização do repositório

| Caminho | Conteúdo |
|---|---|
| `crates/oito2_foundation/` | Tipos e regras puros; `tests/parity.rs` e `tests/compose.rs`; a fixture de paridade em `tests/fixtures/` |
| `crates/oito2_settings/` | Leitores por plataforma (`src/linux/`, `src/windows/`, `src/macos/`), o observador (`src/watcher.rs`), `examples/print_settings.rs` |
| `.github/workflows/` | `ci.yml`, `parity.yml`, `release.yml` |
| `deny.toml` | Licenças aceitas, advisories ignorados e origens permitidas |
| `docs/` | Esta documentação, em inglês (`en/`) e português (`pt-br/`) |

## Verificações

Rode todas antes de abrir um pull request; o CI roda as mesmas no Linux, Windows e macOS.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked
cargo deny check
dbus-run-session -- cargo test --workspace --all-features --locked   # Linux; plain `cargo test` elsewhere
cargo test --workspace --no-default-features --locked
cargo +1.90 check --workspace --all-targets --all-features --locked
```

## Regras de código

- **Cabeçalho de licença.** Todo arquivo `.rs` escrito à mão começa com o cabeçalho GPL-3.0 usado em `crates/*/src/lib.rs` ("Copyright (C) 2026  OITO2").
- **Comentários em inglês**, descrevendo o que o código faz.
- **Documentação.** Todo item público tem rustdoc (`#![deny(missing_docs)]`).
- **Lints.** `unsafe_code` é proibido no workspace; os lints `unwrap_used`, `panic` e `todo` do clippy são negados (relaxados nos testes pelo `clippy.toml`). O `oito2_settings` declara os mesmos lints com `unsafe_code = "deny"`, e só `src/macos/watch.rs` permite código unsafe, cada bloco com um comentário `// SAFETY:`.
- **Contrato JSON.** A saída `serde` continua idêntica à que os plugins do oito2_uikit enviam. Campos novos são opcionais e aditivos; todo tipo com contrato `serde` tem teste de serialização.
- **Armazenamentos do Linux** passam pelo GIO (`gio`/`glib` 0.18, a linha do crate `gtk` que o Tauri usa) e pelo portal de configurações via `gio::DBusConnection`: sem subprocesso `gsettings` e sem `zbus`.

## Fixture de paridade

`crates/oito2_foundation/tests/fixtures/foundation.json` e `manifest.json` são cópias das fixtures do oito2_ui. O workflow semanal **Parity** os compara com `oito2_ui@main` e falha quando o oito2_ui mudou. Para atualizar, copie os dois arquivos de novo, registre o commit de origem em `tests/fixtures/README.md`, porte a mudança e rode as verificações.

## Testes manuais

O CI não consegue trocar o tema de um desktop, então os leitores e observadores também são verificados à mão. Rode:

```bash
cargo run -p oito2_settings --example print_settings --features watch
```

| SO | Mude com o exemplo rodando | Esperado |
|---|---|---|
| Linux (família GNOME) | Layout dos botões da janela, destaque, estilo claro/escuro, alto contraste | Um bloco `--- changed ---` por mudança, com os valores novos |
| Linux (KDE, Xfce, COSMIC) | Botões da barra de título, destaque, esquema de cores | Igual |
| Windows | Cor de destaque, modo do app (claro/escuro), tema de contraste | Igual |
| macOS | Destaque, aparência, "Aumentar contraste" | O exemplo imprime os valores atuais; as mudanças só chegam dentro de um app que roda o run loop principal (veja [Como o oito2_rs funciona](concepts/how-oito2_rs-works.md)) |

## Pull requests

- Um assunto por pull request, com testes para o comportamento alterado.
- Atualize o `CHANGELOG.md` do crate em `[Unreleased]` para mudanças visíveis ao usuário, e a documentação em inglês e português.
- Preencha o modelo de pull request.

## Publicação de versões

Os dois crates compartilham a versão do workspace (`[workspace.package] version` no `Cargo.toml`).

1. Defina a nova versão no `Cargo.toml`, mova a seção `[Unreleased]` de cada `CHANGELOG.md` para a versão com a data e faça o commit.
2. Crie a tag no commit e envie a tag:

   ```bash
   git tag -a v0.1.0 -m "Release 0.1.0"
   git push origin v0.1.0
   ```

3. O workflow **Release** confere se a tag é igual à versão do workspace, publica no crates.io, por trusted publishing, os crates cuja versão ainda não está lá, e cria um rascunho de GitHub Release com as seções dos changelogs. Publique o rascunho depois de revisá-lo.

O trusted publishing só pode ser configurado para um crate que já existe no crates.io, então a primeira versão de um crate novo é publicada à mão, antes do envio da tag:

1. Crie um token de API do crates.io de curta duração, com os escopos `publish-new` e `publish-update`, e rode `cargo login`.
2. Publique em ordem de dependência, a partir do commit da versão:

   ```bash
   cargo publish -p oito2_foundation --locked
   cargo publish -p oito2_settings --locked
   ```

3. Revogue o token. Na página de cada crate no crates.io, adicione um trusted publisher (*Settings → Trusted Publishing*): owner `oito2`, repositório `oito2_rs`, workflow `release.yml`, ambiente `release`. O repositório no GitHub precisa de um ambiente `release`.
4. Envie a tag como no passo 2 acima: o workflow encontra a versão já publicada, pula a publicação e cria o rascunho da versão.
