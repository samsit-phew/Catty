use serde::{Serialize, Deserialize};
use ratatui::style::Color;
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub colors: ColorConfig,
    pub keybinds: KeybindConfig,
    pub visualizer: VisualizerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    pub foreground: String,
    pub background: String,
    pub accent: String,
    pub visualizer_foreground: String,
    pub visualizer_background: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindConfig {
    pub quit: String,
    pub play_pause: String,
    pub next: String,
    pub previous: String,
    pub shuffle: String,
    pub volume_up: String,
    pub volume_down: String,
    pub select: String,
    pub clear: String,
    pub seek_forward: String,
    pub seek_backward: String,
    pub help: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizerConfig {
    pub bar_count: usize,
    pub smoothing: f32,
}

/* ---------------------- Default Implementations ---------------------- */

impl Default for Config {
    fn default() -> Self {
        Self {
            colors: ColorConfig::default(),
            keybinds: KeybindConfig::default(),
            visualizer: VisualizerConfig::default(),
        }
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            foreground: "white".into(),
            background: "black".into(),
            accent: "cyan".into(),
            visualizer_foreground: "LightBlue".into(),
            visualizer_background: "black".into(),
        }
    }
}

impl Default for KeybindConfig {
    fn default() -> Self {
        Self {
            quit: "q".into(),
            play_pause: "space".into(),
            next: "n".into(),
            previous: "p".into(),
            shuffle: "s".into(),
            volume_up: "+".into(),
            volume_down: "-".into(),
            select: "enter".into(),
            clear: "c".into(),
            seek_forward: "l".into(),
            seek_backward: "h".into(),
            help: "?".into(),
        }
    }
}

impl Default for VisualizerConfig {
    fn default() -> Self {
        Self {
            bar_count: 50,
            smoothing: 0.7,
        }
    }
}

/* ---------------------- Config Load and Create ---------------------- */

impl Config {
    pub fn load() -> Self {
        let path = "config.toml";

        if Path::new(path).exists() {
            let content = fs::read_to_string(path).unwrap_or_default();

            if let Ok(cfg) = toml::from_str::<Config>(&content) {
                return cfg;
            } else {
                eprintln!("config.toml found but invalid. Regenerating with defaults.");
            }
        }

        let default = Config::default();
        let serialized = toml::to_string_pretty(&default)
            .expect("Failed to serialize default config");

        fs::write(path, serialized)
            .expect("Failed to write default config.toml");

        default
    }

    pub fn parse_color(color_str: &str) -> Color {
        match color_str.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "gray" | "grey" => Color::Gray,
            "darkgray" | "darkgrey" => Color::DarkGray,
            "lightred" => Color::LightRed,
            "lightgreen" => Color::LightGreen,
            "lightyellow" => Color::LightYellow,
            "lightblue" => Color::LightBlue,
            "lightmagenta" => Color::LightMagenta,
            "lightcyan" => Color::LightCyan,
            s if s.starts_with('#') && s.len() == 7 => {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&s[1..3], 16),
                    u8::from_str_radix(&s[3..5], 16),
                    u8::from_str_radix(&s[5..7], 16),
                ) {
                    Color::Rgb(r, g, b)
                } else {
                    Color::White
                }
            }
            _ => Color::White,
        }
    }
}

