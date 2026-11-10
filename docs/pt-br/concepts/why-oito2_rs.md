🌐 [English](../../en/concepts/why-oito2_rs.md) | **Português** | 🏠 [Índice](../index.md)

---

# Por que o oito2_rs

Os apps do oito2 seguem o desktop em que rodam: os botões da janela ficam onde o desktop os põe, no estilo do desktop, com a cor de destaque, o contraste e o esquema claro ou escuro dele. As regras que decidem isso não são triviais — dez desktops Linux guardam as preferências em lugares diferentes, com formatos e padrões diferentes — e foram escritas primeiro em Dart, para os pacotes Flutter do oito2_ui.

O lado Rust do oito2 precisa das mesmas regras duas vezes:

- **oito2_uikit** — os plugins Tauri dele leem o sistema para um webview Svelte.
- **oito2_iced** — um app Rust nativo que desenha a própria moldura da janela.

Manter uma cópia por consumidor deixaria as duas divergirem. O oito2_rs é a cópia Rust única:

- **Uma implementação** da detecção, da leitura dos layouts e da composição, testada contra a fixture que o oito2_ui exporta, para que Dart e Rust concordem caso a caso.
- **Sem dependência de UI** — sem Tauri, sem toolkit: as regras puras (`oito2_foundation`) não têm nenhuma dependência de sistema, e os leitores (`oito2_settings`) dependem só do GIO no Linux e dos frameworks do sistema nos demais.
- **O JSON dos plugins** — a saída `serde` é igual à que os plugins Tauri já enviam, então o código do webview não muda quando os plugins passarem a usar estes crates.
- **Observação sem loop principal** — o observador do Linux roda o próprio contexto principal do GLib, então funciona dentro do Tauri (que roda o GTK) e dentro de um app winit ou Iced (que não roda).

## Para quem é

Os crates foram escritos para estes consumidores:

| Consumidor | Usa |
|---|---|
| `tauri-plugin-oito2-settings` (oito2_uikit) | `read_raw_settings` e `watch`, repassando o `RawSettings` ao webview |
| `tauri-plugin-oito2-window` (oito2_uikit) | `DecorationMode`, `window_menu` |
| oito2_iced | `read_system_settings`, `watch` e os tipos do foundation |

## O que ele não faz

- Não desenha nada nem altera nenhuma janela: informa valores.
- Não grava configurações.
- Lê só as configurações que o oito2 segue; não é uma biblioteca genérica de configurações.
