🌐 [English](../../en/concepts/glossary.md) | **Português** | 🏠 [Índice](../index.md)

---

# Glossário

| Termo | Significado |
|---|---|
| **Cor de destaque** | A cor de realce escolhida nas configurações do sistema, informada como `#RRGGBB`. |
| **Layout dos botões** | Quais botões da janela (minimizar, maximizar, fechar) ficam antes e depois do título: `WindowButtonLayout { leading, trailing }`. |
| **Esquema de cores** | A preferência clara ou escura do sistema: `ColorScheme::Light`, `ColorScheme::Dark`, ou `None` quando o sistema não declara preferência. |
| **Compor** | `SystemSettings::compose`: transformar um `RawSettings` no `SystemSettings` de uma plataforma. |
| **Debounce** | Esperar as mudanças pararem de chegar (150 ms) antes de ler de novo, para que uma rajada gere um único callback. |
| **Modo de decoração** | Como a moldura de uma janela é desenhada: `native`, `undecorated` ou `gtk-headerless`. |
| **Ambiente de desktop** | O desktop Linux da sessão (GNOME, KDE Plasma, Xfce, ...), detectado por `XDG_CURRENT_DESKTOP` e `DESKTOP_SESSION`. |
| **Emissor** | A thread do observador que faz o debounce, lê, deduplica e chama o callback. |
| **Fallback** | `SystemSettings::fallback`: as configurações de uma plataforma sem nada lido. |
| **GSettings** | O armazenamento de configurações do GNOME (geralmente sobre o dconf), lido pelo GIO. |
| **Run loop principal** | O loop de eventos da thread principal de um aplicativo macOS, que entrega as notificações do sistema. |
| **Fixture de paridade** | `foundation.json`, exportado pelo oito2_ui com o resultado esperado de cada caso das regras. |
| **Configurações brutas** | `RawSettings`: os valores como o sistema os guarda, antes de qualquer regra. |
| **Portal de configurações** | `org.freedesktop.portal.Settings`, a interface do XDG desktop portal que expõe as chaves `org.freedesktop.appearance` via D-Bus. |
| **Configurações do sistema** | `SystemSettings`: o que um app segue — plataforma, desktop, estilo dos controles, layout dos botões, destaque, esquema de cores, alto contraste. |
| **Estilo dos controles da janela** | A família visual dos botões da janela: `gnome`, `breeze`, `xfwm`, `cosmic`, `windows`, `macos` ou `none`. |
