# Catty — Terminal Music Player

Catty is a compact terminal music player with a visualizer, playlist, and an extensible UI.

Highlights
- Keyboard-driven UI with configurable keybinds via `config.toml` (see `config.sample.toml`).
- Visualizer and clean list view.
- Command palette (toggle with `t` by default) to quickly search and play tracks — similar to VS Code / Lapce.
- Optional lyrics panel toggleable with `y`.

Quickstart
1. Copy and edit the example config: `cp config.sample.toml config.toml` and tweak keybinds/colors.
2. Build and run: `cargo build --release && ./target/release/catty`

Command palette (quick overview)
- Open: press the key mapped to `toggle_palette` (default: `t`).
- Type to filter tracks by title. Use Up/Down to navigate, Enter to play.

Customization
- Colors accept named terminal colors or hex `#RRGGBB` in the config.

For more, see `config.sample.toml` in the repository root.