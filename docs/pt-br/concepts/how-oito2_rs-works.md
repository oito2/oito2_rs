🌐 [English](../../en/concepts/how-oito2_rs-works.md) | **Português** | 🏠 [Índice](../index.md)

---

# Como o oito2_rs funciona

```text
system stores ──read──▶ RawSettings ──compose(platform)──▶ SystemSettings
      │                     ▲
      └──change──▶ watcher ─┘ (debounce, read again, deduplicate, callback)
```

## 1. Leitura: `RawSettings`

Um leitor coleta os valores como o sistema os guarda, sem decidir nada:

- **Linux** — `XDG_CURRENT_DESKTOP` e `DESKTOP_SESSION`; as chaves GSettings dos schemas do GNOME, Cinnamon e MATE; `kwinrc`, `kdeglobals`, `xfwm4.xml` e os arquivos de configuração do COSMIC em `XDG_CONFIG_HOME`; as chaves `accent-color`, `contrast` e `color-scheme` do portal de configurações.
- **Windows** — as cores de destaque e de fundo do `UISettings`, e a flag de alto contraste do `AccessibilitySettings`.
- **macOS** — `NSColor.controlAccentColor`, a preferência "Aumentar contraste" e o user default `AppleInterfaceStyle`.

O resultado é um `RawSettings`. Veja [Fontes das configurações](../reference/settings-sources.md) para cada chave.

## 2. Composição: `SystemSettings`

`SystemSettings::compose(&raw, platform)` aplica as regras do oito2_ui:

1. No Linux nativo, o ambiente de desktop é detectado pelas variáveis de sessão.
2. O estilo dos controles da janela segue a plataforma e o desktop (Breeze no KDE, Xfwm no Xfce, COSMIC no COSMIC, GNOME nos demais desktops Linux, os estilos do Windows e do macOS nesses sistemas, nenhum na web e em sistemas touch).
3. O layout dos botões vem do armazenamento do próprio desktop, ou do padrão do estilo.
4. O destaque, o alto contraste e o esquema de cores vêm primeiro do armazenamento do desktop, depois do valor do sistema, com os fallbacks de cada desktop.

`SystemSettings::fallback(platform, desktop)` dá o mesmo resultado sem nada lido. Veja [Regras de composição](../architecture/composition.md).

## 3. Observação

`watch(callback)` inicia duas threads:

- uma **thread da plataforma**, que escuta as notificações de mudança do sistema — sinais do GSettings, monitores de arquivo e o sinal do portal no Linux; eventos do `UISettings` e do `AccessibilitySettings` no Windows; observadores de notificação no macOS — e só avisa que "algo mudou";
- uma **thread emissora**, que espera 150 ms sem novos avisos, lê as configurações de novo, compara com as últimas e chama o callback só quando elas diferem.

Descartar o `SettingsWatcher` retornado para as duas threads. Veja [Observadores](../architecture/watchers.md).

### De onde vêm as mudanças, por plataforma

| SO | O que precisa do aplicativo |
|---|---|
| Linux | Nada: o observador roda o próprio contexto principal do GLib |
| Windows | Nada: o Windows Runtime dispara os eventos em threads próprias |
| macOS | Um run loop principal em execução, que todo aplicativo AppKit tem |
