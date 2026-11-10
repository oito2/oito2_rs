🌐 [English](../../en/architecture/watchers.md) | **Português** | 🏠 [Índice](../index.md)

---

# Observadores

`watch(on_change)` (feature `watch`) devolve um `SettingsWatcher` criado por `SettingsWatcher::start` em `crates/oito2_settings/src/watcher.rs`, com uma parte por plataforma em `{linux,windows,macos}/watch.rs`.

## Início

1. As configurações são lidas uma vez, de forma síncrona, como referência.
2. A **thread emissora** (`oito2-settings-emit`) começa.
3. A **thread da plataforma** (`oito2-settings-watch`) começa e anexa as fontes; `watch` só retorna depois que ela avisa que estão anexadas, então nenhuma mudança logo depois do retorno se perde.

## Eventos de mudança

A parte da plataforma só envia `Event::Changed` por um canal `std::sync::mpsc`:

| SO | Fontes |
|---|---|
| Linux | `changed` de `gio::Settings` para cada schema instalado de `GSETTINGS_KEYS` (cada chave observada é lida uma vez, porque o GSettings só informa chaves já lidas); `gio::FileMonitor` (`WATCH_MOVES`) em cada diretório de configuração existente, filtrado pelos nomes observados, o que pega gravações no lugar e renomeações atômicas; o sinal `SettingChanged` do portal para `org.freedesktop.appearance`. Tudo anexado a um `glib::MainContext` que pertence à thread (`with_thread_default`) e roda com um `MainLoop`. |
| Windows | Handlers de `UISettings.ColorValuesChanged` e `AccessibilitySettings.HighContrastChanged`; a thread mantém os registros vivos, e o Windows Runtime chama os handlers em threads próprias. |
| macOS | Observadores com bloco para `NSSystemColorsDidChangeNotification` (centro padrão), `AppleInterfaceThemeChangedNotification` (centro distribuído) e `NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification` (centro do workspace). Blocos sem fila rodam na thread que posta, então as notificações chegam pelo run loop principal do aplicativo. |

## Emissor

Depois de um evento, o emissor espera até não chegar nenhum evento por 150 ms (`recv_timeout`), lê as configurações e chama `on_change` só quando elas diferem do último valor visto. Ele para com `Event::Stop` ou quando todos os remetentes somem.

## Parada

`Drop for SettingsWatcher`:

1. Para a parte da plataforma — no Linux `MainContext::invoke(|| main_loop.quit())`, que roda quando o loop itera, então um pedido de parada antes do loop começar não se perde; no Windows e no macOS uma mensagem num canal — e espera a thread dela, que remove os handlers ou observadores.
2. Envia `Event::Stop` e espera o emissor.

A espera é pulada quando a thread atual é a que seria esperada, então descartar o observador dentro do próprio callback não trava. `SettingsWatcher` é `Send + Sync`, então pode ficar no estado compartilhado do aplicativo.

## Limites conhecidos

- No Linux, diretórios de configuração criados depois do início do `watch` não são monitorados.
- No macOS, nada é informado sem um run loop principal em execução (um programa simples de linha de comando).
