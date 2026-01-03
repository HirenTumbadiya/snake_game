# Snake Game (macroquad + Rust)

Small grid-based snake game implemented with `macroquad`.

![screenshot](assets/game.png)

Features
- **Wrap-around:** snake wraps when crossing grid edges.
- **Food:** red food spawns randomly (never on the snake).
- **Growth:** snake grows when eating food.
- **No reverse:** directional input cannot reverse directly into the snake's body.
- **Game over:** colliding with self shows a Game Over message; press `R` to restart.

Build & Run (development)

PowerShell (Windows):
```powershell
cargo run
```

Release build (Windows executable)
```powershell
cargo build --release
# The built exe will be at:
# target\release\snake_game.exe
```

Releases / Download button for Windows
- You can publish a GitHub Release and attach the built `snake_game.exe` (or a ZIP that contains it).
- Once uploaded, the asset is downloadable via a predictable URL:

`https://github.com/HirenTumbadiya/snake_game/releases/latest/download/<ASSET_NAME>`

Example Markdown button linking to the latest attached exe:

```markdown
[Download for Windows](https://github.com/HirenTumbadiya/snake_game/releases/latest/download/snake_game.exe)
```