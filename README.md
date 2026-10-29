<div align="center">
  <img src="assets/logo.svg" width="96" alt="coverwin">
  <h1>coverwin</h1>
  <p>Converta imagens direto do clique direito no Windows Explorer — como o 7-Zip faz com arquivos compactados.</p>
  <p>
    <a href="https://github.com/eupsa/coverwin/releases/latest"><img src="https://img.shields.io/github/v/release/eupsa/coverwin" alt="release"></a>
    <a href="LICENSE"><img src="https://img.shields.io/github/license/eupsa/coverwin" alt="licença MIT"></a>
    <img src="https://img.shields.io/badge/Windows-10%20%7C%2011-0078d4" alt="Windows">
    <img src="https://img.shields.io/badge/Rust-Tauri%202-orange" alt="Rust + Tauri">
  </p>
</div>

## Recursos

- **Clique direito → Converter para → PNG / JPG / WEBP / AVIF / BMP / GIF / TIFF / ICO**
- Conversão em lote de vários arquivos selecionados
- Nunca sobrescreve: gera `foto (1).png` se já existir
- App desktop com arrastar e soltar, prévias, qualidade ajustável e pasta de saída
- Visual Fluent 2 com Mica, tema claro e escuro
- Instalação por usuário, sem precisar de administrador

## Instalação

Baixe o `coverwin_x.y.z_x64-setup.exe` na [última release](https://github.com/eupsa/coverwin/releases/latest) e execute.
O instalador não é assinado: no aviso do SmartScreen, clique em **Mais informações → Executar assim mesmo**.

No Windows 11 o menu fica em **Mostrar mais opções** (Shift+F10).

### CLI

```powershell
coverwin-cli convert foto.png outra.jpg --to webp -q 80
```

## Stack

| Parte | Tecnologia |
|---|---|
| Núcleo / CLI | Rust |
| Imagem | crates `image` (AVIF via rav1e), `webp` |
| App | Tauri 2 + SvelteKit + Svelte 5 + TypeScript |
| UI | Bits UI + Tailwind CSS 4 com tokens Fluent 2 |
| Menu de contexto | Registro do Windows (`ExtendedSubCommandsKey`) |
| Instalador | NSIS |

## Estrutura

```
crates/
  core/    lógica de conversão
  cli/     coverwin-cli.exe (usado pelo menu)
app/       app Tauri (Svelte)
scripts/   instalação/remoção do menu
```

## Desenvolvimento

Requisitos: Rust stable, [NASM](https://nasm.us) no PATH (encoder AVIF), Node.js LTS, pnpm, [pré-requisitos do Tauri](https://tauri.app/start/prerequisites/).

```powershell
cargo test
cd app; pnpm install; pnpm tauri dev   # app em modo dev
pnpm tauri build                       # instalador em target/release/bundle/nsis
```

Menu de clique direito sem instalador (usuário atual):

```powershell
cargo build --release
powershell -ExecutionPolicy Bypass -File scripts\install-menu.ps1
powershell -ExecutionPolicy Bypass -File scripts\uninstall-menu.ps1   # remover
```

## Roadmap

- Conversão de vídeo (FFmpeg como executável separado)
- Menu moderno do Windows 11 (`IExplorerCommand` + MSIX)

## Contribuindo

Issues e pull requests são bem-vindos.

## Licença

[MIT](LICENSE) © Pedro Araujo
