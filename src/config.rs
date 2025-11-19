use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Config {
    pub colors: ColorConfig,
    pub keybinds: KeybindConfig,
    pub visualizer: VisualizerConfig,
}

#[derive(Debug, Clone)]
pub struct ColorConfig {
    pub foreground: String,
    #[allow(dead_code)]
    pub background: String,
    pub accent: String,
    pub visualizer_foreground: String,
    pub visualizer_background: String,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct VisualizerConfig {
    pub bar_count: usize,
    pub smoothing: f32,
}

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
            foreground: "white".to_string(),
            background: "black".to_string(),
            accent: "cyan".to_string(),
            visualizer_foreground: "LightBlue".to_string(),
            visualizer_background: "black".to_string(),
        }
    }
}

impl Default for KeybindConfig {
    fn default() -> Self {
        Self {
            quit: "q".to_string(),
            play_pause: "space".to_string(),
            next: "n".to_string(),
            previous: "p".to_string(),
            shuffle: "s".to_string(),
            volume_up: "+".to_string(),
            volume_down: "-".to_string(),
            select: "enter".to_string(),
            clear: "c".to_string(),
            seek_forward: "l".to_string(),
            seek_backward: "h".to_string(),
            help: "?".to_string(),
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

impl Config {
    pub fn load() -> Self {
        Config::default()
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
