# Patch Force: Runtime Rebellion

Patch Force: Runtime Rebellion is a small Rust WebAssembly browser game built with Macroquad and Trunk. You play a tiny dev commando fighting through a corrupted production codebase where bugs, failing tests, deploy hazards, and merge conflicts have become physical enemies.

The game is an original run-and-gun inspired by classic side-scrolling arcade action.

## Assets

The playable build uses a mix of Rust/Macroquad-generated effects, original generated SFX, and attributed Open Gunner pixel-art sprite sheets by Master484 for upgraded player, enemy, boss, and scrolling background visuals. Source and license details are kept in `ASSET_ATTRIBUTION.md`.

No API keys, tokens, secrets, or `.env` files are required.

## Controls

- A / D or Left / Right: move
- W / Space / Up: jump
- S / Down: crouch
- J or Left Click: shoot
- Keyboard direction keys while shooting: eight-direction aim
- K: return to keyboard aim after mouse aim is active
- R: restart
- Enter: start or restart
- I: instructions from the start screen
- Escape: pause or return from instructions

## Weapons and Pickups

- Patch Rifle: default straight shot
- Spread Diff: three-shot spread
- Refactor Beam: piercing heavy shot
- Hotfix SMG: rapid low-damage fire
- Health pickup: restores health
- Test Shield: temporary invulnerability

## Run Locally

Install the WebAssembly target and Trunk:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Serve the browser build:

```sh
trunk serve
```

Open the local URL Trunk prints, usually `http://127.0.0.1:8080/`.

If Trunk reports `invalid value '1' for '--no-color'`, your shell is exporting `NO_COLOR=1`. Run the command with `NO_COLOR=false`, for example:

```sh
NO_COLOR=false trunk serve
```

## Build for Release

```sh
trunk build --release
```

The static build is written to `dist/`.

Use `NO_COLOR=false trunk build --release` if your local Trunk install rejects `NO_COLOR=1`.

This project uses Trunk hooks to build Macroquad's raw `wasm32-unknown-unknown` artifact and copy it into `dist/` for Macroquad's browser loader.

## Deploy to GitHub Pages

1. Build with the repository public URL:

```sh
trunk build --release --public-url /YOUR_REPOSITORY_NAME/
```

2. Publish the generated `dist/` folder with GitHub Pages.
3. If using GitHub Actions, upload `dist/` as the Pages artifact.

For a user or organization Pages site at the domain root, use:

```sh
trunk build --release --public-url /
```

## Submit to itch.io

1. Run `trunk build --release`.
2. Zip the contents of `dist/`, not the `dist` folder itself.
3. Create an HTML game page on itch.io.
4. Upload the zip.
5. Set the game to run in the browser.

## Challenge Submission Checklist

- Complete start, instructions, game over, and victory screens
- One complete side-scrolling level with Legacy Jungle, CI/CD Factory, and Production Core sections
- Boss fight against the Merge Conflict Mech
- Score, health, lives, current weapon, and boss health HUD
- Four enemy types
- Four weapon types
- Health and shield pickups
- No backend
- No API keys
- No secrets
- No tokens
- No `.env` files
- No copyrighted art, music, characters, names, or branding
- `cargo fmt` run
- `cargo check` passes
- `trunk build --release` passes

## Secrets Warning

Do not commit secrets, API keys, tokens, credentials, or `.env` files. This game is fully static and does not need any private configuration.
