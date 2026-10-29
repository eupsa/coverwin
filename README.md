# coverwin

Converta imagens e vídeos direto do clique direito no Windows Explorer — do mesmo jeito que o 7-Zip e o WinRAR fazem com arquivos compactados.

> 🚧 Em desenvolvimento.

## Recursos (planejados)

- Clique direito em imagem/vídeo → **Converter para → PNG / JPG / WEBP / AVIF / MP4 / GIF…**
- Conversão em lote de vários arquivos selecionados
- App desktop com arrastar e soltar, fila e progresso
- Integração com o menu moderno do Windows 11

## Stack

| Parte | Tecnologia |
|---|---|
| Núcleo / CLI | Rust (stable) |
| Vídeo | FFmpeg (build LGPL, executável separado) |
| Imagem | crates `image`, `webp`, `ravif` |
| App | Tauri 2 + Svelte 5 + TypeScript |
| UI | Bits UI + Tailwind CSS 4 com visual Fluent 2 |
| Menu de contexto | Registro do Windows → `IExplorerCommand` + MSIX |
| Instalador | NSIS |

## Estrutura

```
crates/
  core/    lógica de conversão
  cli/     coverwin-cli.exe (usado pelo menu)
  shell/   extensão do Explorer (IExplorerCommand)
app/       app Tauri (Svelte)
```

## Desenvolvimento

Requisitos: Rust stable, [NASM](https://nasm.us) no PATH (usado pelo encoder AVIF), Node.js LTS, pnpm, [pré-requisitos do Tauri](https://tauri.app/start/prerequisites/).

```bash
cargo test
```

## Licença

A definir. O FFmpeg é distribuído como executável separado sob LGPL.
