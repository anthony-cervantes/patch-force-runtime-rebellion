# Asset Attribution

Patch Force: Runtime Rebellion is an original game. It must not copy Contra characters, names, art, music, level layouts, enemies, or branding.

## Planned Attributed Sprite Sources

The project may use and heavily restyle selected sprites from the Open Gunner asset set as a base for a stronger run-and-gun visual pass.

### Open Gunner Starter Kit

- Author: Master484
- Source: https://opengameart.org/content/open-gunner-starter-kit
- License: CC-BY 3.0 or OGA-BY 3.0
- Attribution requested by author: credit "Master484"; when practical, link to the OpenGameArt profile and website.
- Author website: http://m484games.ucoz.com/

Potential use:
- Player movement and aiming pose references
- Gun/rifle pose references
- Mech/boss reference parts
- Panels, bars, turrets, and sci-fi sprite language

Required project treatment:
- Recolor and restyle imported sprites into the Patch Force palette.
- Avoid copying Contra-like red/blue commando silhouettes.
- Avoid using Open Gunner level compositions directly.
- Keep Patch Force names, enemies, boss identity, and theme original.

### Open Gunner Expansion Pack 1

- Author: Master484
- Source: https://opengameart.org/content/open-gunner-expansion-pack-1
- License: CC-BY 3.0 or OGA-BY 3.0
- Attribution requested by author: credit "Master484"; when practical, link to the OpenGameArt profile and website.
- Author website: http://m484games.ucoz.com/

Potential use:
- Extra player movement references
- Turret/enemy references
- Forest/jungle tile references

### Open Gunner Expansion Pack 2

- Author: Master484
- Source: https://opengameart.org/content/open-gunner-expansion-pack-2
- License: CC-BY 3.0 or OGA-BY 3.0
- Attribution requested by author: credit "Master484"; when practical, link to the OpenGameArt profile and website.
- Author website: http://m484games.ucoz.com/

Potential use:
- Additional enemy, boss, and projectile references if selected during the replacement pass.

## Current Build Usage

The playable build now loads selected Open Gunner PNG sprite sheets from `assets/open_gunner/` at startup and applies a transparency/keying pass in Rust before drawing them with Macroquad. These assets are used for the player, boss, several enemies, and scrolling background set dressing. Rust/Macroquad-generated primitives are still used for HUD text, particles, projectiles, collision platforms, pickups, and extra Patch Force overlays.
