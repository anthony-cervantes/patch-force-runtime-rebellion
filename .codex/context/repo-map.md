# Repo Map

## Snapshot
- Project: Patch Force: Runtime Rebellion
- Type: Browser game project (single Rust application)
- Primary languages: Rust (game logic), HTML/CSS/JS scaffolding for browser entry
- Package manager: Cargo
- Test framework: none configured in repo metadata
- Runtime/deploy target: WebAssembly in browser via Trunk (`wasm32-unknown-unknown`) and GitHub Pages

## Architecture
Single Rust crate with game systems split under `src/` (player, enemies, bosses, projectiles, levels, UI, rendering). `Trunk.toml` handles build hook wiring for Macroquad wasm artifact placement. `index.html` is the browser entry. Static assets under `assets/`.

## Task routing
| Task type | Start here | Then inspect | Avoid |
|---|---|---|---|
| Gameplay behavior | `src/` | `src/game.rs`, `src/player.rs`, `src/enemy.rs` | `dist/`, `target/`, `assets/` unless art changes are required |
| Rendering/visual changes | `src/sprite*.rs`, `assets/`, `index.html` | `Trunk.toml`, `ASSET_ATTRIBUTION.md` | `target/`, `dist/`, `.trunk/` |
| Build/run loop | `README.md` and `Trunk.toml` | `Cargo.toml`, workflow file | `Cargo.lock` unless dependency issue |
| Deployment | `.github/workflows/deploy-github-pages.yml` | `Trunk.toml`, README deploy section | `src/` if no gameplay changes |
| Content/process docs | `README.md` and `ASSET_ATTRIBUTION.md` | `.gitignore`, existing commit history context | build artifacts |

## Top-level directories
| Path | Purpose | Notes |
|---|---|---|
| `src/` | Core game code | Most development work lives here |
| `assets/` | Game sprites and visual inputs | Attribution and palette considerations apply |
| `dist/` | Build output (generated) | Do not edit; do not rely on file-level diffs here |
| `target/` | Rust build output (generated) | Ignore unless debugging toolchain issues |
| `.github/` | CI workflow for Pages deployment | Primarily `deploy-github-pages.yml` |
| `.playwright-cli/` | Tooling artifacts | Ignore by default |
| `.cargo/` | Cargo config | Rarely needed; usually unchanged |
| `.codex/` | Repo guidance metadata | Created by this bootstrap |
| `js/` | Empty placeholder | No manifest/build tooling present |

## Important files
| File | Purpose |
|---|---|
| `Cargo.toml` | Crate metadata and dependencies |
| `Trunk.toml` | Build/serve config and post-build hook |
| `README.md` | Setup, run, and deploy instructions |
| `.github/workflows/deploy-github-pages.yml` | CI publish pipeline |
| `ASSET_ATTRIBUTION.md` | Asset licensing and treatment constraints |
| `index.html` | Browser entry file |
| `src/main.rs` | App bootstrapping and entrypoint |
| `src/game.rs` | Primary game loop and state flow |
| `src/ui.rs` | UI and overlay behaviors |

## Commands
| Task | Command | Scope | Confidence |
|---|---|---|---|
| Serve locally | `trunk serve` | Browser game runtime | High |
| Serve with NO_COLOR workaround | `NO_COLOR=false trunk serve` | Local run on shells exporting NO_COLOR | High |
| Build release | `trunk build --release` | Entire app output in `dist/` | High |
| Build for Pages repo path | `NO_COLOR=false trunk build --release --public-url /patch-force-runtime-rebellion/` | GitHub Pages deployment artifacts | High |
| Check compilation | `cargo check` | Full crate compile | High |
| Format | `cargo fmt` | Source formatting | High |
| Host build sanity (inferred) | `cargo build` | Single-pass host validation | Medium |

## Test strategy
Start with `cargo check` for functional edits. Use `trunk build --release` as the primary integration build verification. Expand tests only if a test harness is added (currently none).

## Generated/vendor/build paths
`target/`, `dist/`, `.trunk/`, `.playwright-cli/`, `*.wasm`, `*.dSYM/`, `.DS_Store`.

## Risk areas
- Asset usage and attribution drift from source attributions.
- Browser wasm build breakages from Trunk hook/asset pipeline changes.
- Deployment path mismatches between local/public URLs.
- Gameplay balance regressions from edits across multiple `src/` modules.

## Nested AGENTS.md files
| Path | Reason |
|---|---|
| (none) | Single-language single-crate repo; no module boundaries currently requiring overrides |

## Unknowns
- No test framework or test command is currently configured in manifests.
- No explicit lint command is declared outside `cargo fmt`.
