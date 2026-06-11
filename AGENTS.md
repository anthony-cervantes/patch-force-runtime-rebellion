# AGENTS.md

## Purpose
This repository is a small Rust/WebAssembly browser game built with `macroquad` and `trunk`.

## Token budget rules
- Start with `.codex/context/repo-map.md` before exploring.
- Prefer `git status --short`, `git ls-files`, and targeted `rg`.
- Read only files directly relevant to the current task.
- Do not inspect build outputs, dependency caches, minified assets, or generated artifacts unless task-relevant.
- Use narrowest verification first (single file/command).
- Ask before running full test suites, dependency installs, migrations, broad refactors, or repo-wide scans.
- Ask before full scans, repo-wide formatting, or heavy CI sweeps.
- Do not use subagents for small/local tasks unless explicitly requested.

## Repo map
See [`.codex/context/repo-map.md`](./.codex/context/repo-map.md).

## Common commands

### Setup / install
- `rustup target add wasm32-unknown-unknown`
- `cargo install trunk`

### Local dev
- `trunk serve`
- `NO_COLOR=false trunk serve`

### Build
- `trunk build --release`
- `NO_COLOR=false trunk build --release`
- `NO_COLOR=false trunk build --release --public-url /patch-force-runtime-rebellion/`

### Maintenance / quality
- `cargo fmt`
- `cargo check`
- `cargo build` (inferred for host build only)
- `cargo test` (inferred; repository currently has no test command declared)

## Workflows

### Bug fix workflow
1. Reproduce with a narrow run (`trunk serve` if needed).
2. Inspect only the affected game logic/state files in `src/`.
3. Patch minimally and run `cargo check`.
4. Run `trunk build --release` before proposing changes.

### Feature workflow
1. Update `README.md` and/or in-code comments for visible behavior changes.
2. Edit only target module(s) in `src/`.
3. Validate with `cargo check`.
4. Confirm build via `trunk build --release`.

### Refactor workflow
1. Preserve `main` gameplay loops and API boundaries where used across files.
2. Refactor one module at a time.
3. Ensure no behavioral regressions by comparing build and core flow.
4. Run `cargo fmt` and `trunk build --release`.

### Test repair workflow
1. Use narrowest test or check first.
2. Fix the immediate failure.
3. Re-run only the previously failing check.
4. Expand scope only after that passes.

### PR review workflow
1. Confirm touched files are under the expected module path.
2. Ensure `Cargo.toml` dependencies remain minimal.
3. Confirm generated/build paths remain unchanged.
4. Verify `cargo fmt` and `trunk build --release` reasoning.

## Safety and constraints
- No API/server/backend assumptions; game is static.
- No secrets, API keys, tokens, credentials, or `.env` files should be introduced.
- Dist/target artifacts are build outputs only; do not commit them.
- Follow asset attribution requirements in `ASSET_ATTRIBUTION.md`.
- Preserve browser build compatibility for `wasm32-unknown-unknown`.
- If touching GitHub Pages deployment, keep public URL alignment with Trunk settings.

## Generated and ignored paths
Default avoid: `target/`, `dist/`, `.trunk/`, `.playwright-cli/`, `*.wasm`, `*.dSYM/`, build artifacts.

## Done means
- Minimal diff limited to requested area.
- Local reasoning/validation for requested scope is documented.
- Summary includes changed files.
- Remaining risks and follow-up items are clearly listed.
