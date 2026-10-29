# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# coverwin

Conversor de imagens e vídeos integrado ao menu de clique direito do Windows Explorer (como o 7-Zip/WinRAR fazem com arquivos compactados), com um app desktop para conversões em lote.

## Comandos

```bash
cargo build                                   # workspace inteiro
cargo test                                    # todos os testes
cargo test -p coverwin-core converts_to       # um teste (filtro por nome)
cargo clippy --all-targets                    # deve ficar sem warnings
cargo fmt                                     # rodar antes de commitar
cargo run -p coverwin-cli -- convert a.png b.jpg --to webp -q 80

# app (dentro de app/)
pnpm.cmd tauri dev                            # app em dev (hot reload)
pnpm.cmd check                                # svelte-check
pnpm.cmd exec biome check --write src         # lint + format do frontend
```

- No PowerShell deste PC o shim `pnpm.ps1` sai sem fazer nada — use **`pnpm.cmd`**.
- Rode `tauri dev` pelo painel Terminal (`run_in_terminal`), não pelas ferramentas Bash/PowerShell (ver nota de MSIX abaixo).

Ambiente (Windows):
- Rust instalado via rustup em `~/.cargo/bin`; **NASM** em `%LOCALAPPDATA%\bin\NASM` — obrigatório para compilar `rav1e` (encoder AVIF). Se `cargo`/`nasm` não forem encontrados num shell, prefixe o PATH com esses dois diretórios.
- MSVC Build Tools 2022 já instalado.

## Arquitetura atual

Workspace Cargo (`Cargo.toml` na raiz, `members = ["crates/*", "app/src-tauri"]`, resolver 3, edição 2024; `[profile.release]` fica na raiz):

- `crates/core` (`coverwin-core`) — biblioteca; toda a lógica de conversão. API pública: `Format` (enum + `FromStr`, aceita `jpeg`/`tif`), `convert(input, format, quality, out_dir: Option<&Path>) -> PathBuf`, `Error`.
  - Decodifica com `image::open` e salva em `out_dir` ou **ao lado do original**, nunca sobrescreve (`free_path` gera `nome (1).ext`, `nome (2).ext`…).
  - Casos especiais: JPG remove alfa; WebP usa o crate `webp` (lossy — o encoder do `image` é só lossless); AVIF usa `AvifEncoder` do `image` (que usa `ravif`/`rav1e` por dentro — não adicionar `ravif` direto); ICO reduz para ≤256px.
  - Decode de AVIF **não** está habilitado (exigiria feature `avif-native`/dav1d).
- `crates/cli` (`coverwin-cli`) — fino wrapper `clap` sobre o core. Processa arquivos em sequência, continua após falhas e sai com `ExitCode::FAILURE` se algum falhar (o menu de contexto vai depender disso).

- `scripts/install-menu.ps1` / `uninstall-menu.ps1` — menu de contexto fase 1: submenu cascata em `HKCU\...\SystemFileAssociations\.<ext>\shell\coverwin` (`MUIVerb` + `ExtendedSubCommandsKey=coverwin.menu.<fmt>`, com os itens em `HKCU\Software\Classes\coverwin.menu.<fmt>\shell\NNfmt\command`; a forma `SubCommands=""` com `shell` aninhado não apareceu neste Windows). Cada item chama `conhost.exe --headless coverwin-cli.exe convert "%1" --to <fmt>` (sem janela; erros ficam invisíveis por ora). Explorer invoca uma vez por arquivo selecionado. Aponta por padrão para `target\release\coverwin-cli.exe` — rodar `cargo build --release` antes; listas de extensões estão duplicadas nos dois scripts.

  - **Atenção:** o Claude desktop é um app MSIX; processos iniciados por ele têm gravações em `HKCU\Software\Classes` virtualizadas — o Explorer não as vê. Rode os scripts de menu pelo painel Terminal do app (`run_in_terminal`), que não é virtualizado — nunca pelas ferramentas Bash/PowerShell.

- `app/` — Tauri 2 + SvelteKit 3 (SPA). `src-tauri/src/lib.rs` expõe um único comando `convert(files, format, quality, outDir, channel)`: roda o core em `spawn_blocking` e envia `Progress` (`done` com tamanhos / `failed`) por `tauri::ipc::Channel` arquivo a arquivo. Frontend: `src/routes/+page.svelte` (fila em grade com prévias, painel de opções) + `src/lib/TitleBar.svelte`.
  - Janela sem moldura (`decorations: false`, `transparent`, efeito **Mica**) com barra de título própria (`data-tauri-drag-region` + permissões `core:window:allow-*` em `capabilities/default.json`). Fundo do `body` é transparente — use os tokens translúcidos de `app.css` (`bg-layer`, `bg-card`).
  - Prévias usam `convertFileSrc` → exige feature `protocol-asset` e `assetProtocol` (escopo `**`) + `img-src asset:` na CSP.
  - **SvelteKit 3:** não existe `svelte.config.js` — config (adapter-static com `fallback: "index.html"`) vai em `sveltekit({...})` no `vite.config.js`; `tsconfig.json` estende `$app/tsconfig`. Import de `$lib` não resolveu no svelte-check: use caminho relativo.
  - TypeScript fixo em 6.x: TS 7 ainda não é suportado por SvelteKit 3/svelte-check.

Mudanças que atravessam arquivos:
- `coverwin_core::convert` tem 3 chamadores: `crates/cli/src/main.rs`, `app/src-tauri/src/lib.rs` e o teste em `crates/core/src/lib.rs`.
- A lista de extensões de **entrada** suportadas está duplicada em 4 lugares (sem fonte única): os dois scripts de menu e `INPUT_EXTS` em `+page.svelte`, além do que `image` consegue decodificar. Ao adicionar/remover formato, atualize todos.
- `biome.json` desliga `noUnusedVariables`/`noUnusedImports`/`useConst`/`useImportType` em `.svelte` porque o Biome não enxerga o uso no template.

Planejado (ainda não existe): `crates/shell` (extensão `IExplorerCommand`). Dependências `rayon`, `serde`, `toml`, `tracing*` já estão declaradas mas ainda não usadas.

## Regras

- **Dependências sempre na versão estável mais recente.** Use `cargo add` / `pnpm add` (pegam a última) — nunca chute versões de memória. Sem pré-release.
- Não adicionar dependência para poucas linhas de código.
- Idioma da UI e mensagens de erro: PT-BR (EN depois). Identificadores e commits em inglês, Conventional Commits.
- FFmpeg (vídeo) sempre como executável externo, build **LGPL** — nunca linkar nem usar `--enable-gpl`.

## Stack alvo

- **Núcleo:** Rust stable; imagem via `image` + `webp`; vídeo via FFmpeg sidecar (`std::process::Command`); lote com `rayon`; logs `tracing` + `tracing-appender` em `%LOCALAPPDATA%\coverwin`; config `serde` + `toml` em `%APPDATA%\coverwin`.
- **Menu de contexto:** fase 1 = registro em `HKCU\Software\Classes\SystemFileAssociations\.<ext>\shell\coverwin` (submenu "Converter para → …") chamando `coverwin-cli.exe`, sem admin/COM. Fase 2 = DLL COM `IExplorerCommand` (crate `windows`) + MSIX com identidade esparsa (menu moderno do Windows 11).
- **App:** Tauri 2 + Svelte 5 + SvelteKit (SPA, adapter-static) + TypeScript + Vite. UI com **Bits UI** (headless) + **Tailwind CSS 4** estilizado com tokens do **Fluent 2** (Segoe UI Variable, cantos/cores do Windows 11); ícones Lucide; estado com runes; i18n `paraglide-js`; progresso via `Channel` do Tauri. Plugins: `dialog`, `fs`, `shell`, `updater`, `single-instance`, `notification`.
- **Distribuição:** instalador NSIS (grava o registro do menu na fase 1), MSIX na fase 2, updates via `tauri-plugin-updater` + GitHub Releases.
- **Qualidade:** frontend com Biome e pnpm; CI GitHub Actions + `tauri-action`.

## Roadmap
1. ✅ `core` + `cli` (imagens)
2. ✅ Registro do menu de contexto chamando a CLI
3. ✅ App Tauri: arrastar e soltar, fila com prévias, progresso, pasta de destino
4. Instalador NSIS com menu + sidecar ffmpeg
5. `IExplorerCommand` + MSIX
