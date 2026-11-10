🌐 [English](../../en/getting-started/installation.md) | **Português** | 🏠 [Índice](../index.md)

---

# Instalação

## Requisitos

| SO | Situação | Versão mínima | Dependências em tempo de execução |
|---|---|---|---|
| Linux | Suportado | Qualquer distribuição com GLib 2.56+ | `libglib2.0-0` (GIO); um barramento de sessão D-Bus e o `xdg-desktop-portal` para os valores do portal |
| Windows | Suportado | Windows 10 | — |
| macOS | Suportado | macOS 10.14 | — |

- **Rust 1.90** ou mais novo. Dentro de um clone, o `rustup` instala o toolchain 1.99.0 fixado em `rust-toolchain.toml`.
- **`oito2_foundation`** não precisa de mais nada em nenhum sistema.
- **`oito2_settings` no Linux** liga o GLib e o GIO pelo `pkg-config`, então a compilação precisa dos arquivos de desenvolvimento deles.

### Dependências de compilação no Linux

| Distribuição | Comando |
|---|---|
| Debian, Ubuntu | `sudo apt install libglib2.0-dev pkg-config` |
| Fedora | `sudo dnf install glib2-devel pkgconf-pkg-config` |

Windows e macOS não precisam de pacote de sistema.

## Como adicionar os crates

```bash
# Readers, watcher and the re-exported foundation (oito2_settings::foundation)
cargo add oito2_settings --features watch

# Only the pure rules
cargo add oito2_foundation
```

Ou no `Cargo.toml`:

```toml
[dependencies]
oito2_settings = { version = "0.1", features = ["watch"] }
```

### Features

| Crate | Feature | Padrão | Efeito |
|---|---|---|---|
| `oito2_foundation` | `serde` | sim | (De)serialização JSON de todo tipo público |
| `oito2_settings` | `serde` | sim | (De)serialização JSON; ativa `oito2_foundation/serde` |
| `oito2_settings` | `watch` | não | `watch` e `SettingsWatcher` |

### A partir do Git

Para acompanhar o repositório em vez de uma versão publicada:

```toml
[dependencies]
oito2_settings = { git = "https://github.com/oito2/oito2_rs", features = ["watch"] }
```

## Compilar a partir do código

```bash
git clone https://github.com/oito2/oito2_rs.git
cd oito2_rs
cargo build --workspace --all-features
cargo run -p oito2_settings --example print_settings --features watch
```

O exemplo imprime os valores brutos e as configurações compostas da máquina, e depois cada mudança até você apertar Ctrl+C. No macOS ele só imprime os valores atuais: as mudanças chegam pelo run loop principal de um aplicativo, que um exemplo de linha de comando não roda.

## Desinstalação

Os crates não instalam nada no sistema: remova-os do `Cargo.toml` (`cargo remove oito2_settings`).
