🌐 [English](../../en/troubleshooting/common-issues.md) | **Português** | 🏠 [Índice](../index.md)

---

# Problemas comuns

## Compilação: "The system library `glib-2.0` required by crate `glib-sys` was not found"

Faltam os arquivos de desenvolvimento do GLib ou o `pkg-config`. Instale-os (veja [Instalação](../getting-started/installation.md#dependências-de-compilação-no-linux)): `sudo apt install libglib2.0-dev pkg-config` no Debian/Ubuntu, `sudo dnf install glib2-devel pkgconf-pkg-config` no Fedora.

## Linux: o destaque ou o esquema de cores é sempre `None`

- Os valores do portal precisam de um barramento de sessão D-Bus e de um `xdg-desktop-portal` em execução com um backend para o desktop (`xdg-desktop-portal-gnome`, `-kde`, `-gtk`, ...). Sem eles, as chaves do portal ficam sem valor.
- `color-scheme` 0 significa "sem preferência": a composição então usa o armazenamento do próprio desktop (veja [Regras de composição](../architecture/composition.md#esquema-de-cores)). No Xfce e no LXQt o portal é a única fonte.
- Um destaque só é informado quando o desktop tem um: GNOME 47+ (`accent-color`), KDE (`AccentColor`) ou um portal que implementa `accent-color`.

## Linux: valores ausentes dentro do Flatpak

Dentro do sandbox, o GSettings e os arquivos de configuração do host não ficam visíveis; só os valores do portal são lidos.

## Linux: uma mudança não é informada

O observador monitora os diretórios de configuração que existem quando ele começa. Um diretório criado depois (por exemplo `~/.config/cosmic/com.system76.CosmicTheme.Mode/v1` numa conta nova) só é monitorado depois que o observador é criado de novo.

## macOS: o `watch` não informa nada

As notificações chegam pelo run loop principal do aplicativo. Um programa de linha de comando (como o exemplo `print_settings`) não roda um; um aplicativo AppKit — winit, Iced ou Tauri — roda.

## O `watch` devolve `Error::Unsupported`

A plataforma em execução não é Linux, Windows nem macOS.

## Testes: "fuse init failed" e um teste lento sob `dbus-run-session`

A mensagem vem do `xdg-document-portal` sendo iniciado no barramento de sessão isolado, onde ele não consegue montar o sistema de arquivos FUSE; o teste de leitura espera os serviços do portal nesse barramento. Nenhum dos dois afeta o resultado.
