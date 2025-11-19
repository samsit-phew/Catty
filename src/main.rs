mod audio;
mod config;
mod database;
mod player;
mod ui;
mod visualizer;

use anyhow::Result;
use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

use audio::AudioPlayer;
use database::MusicDatabase;
use player::PlayerState;
use ui::UI;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load();
    
    // Initialize database and scan music
    let mut database = MusicDatabase::new()?;
    database.scan_music_directory()?;

    // Initialize audio player
    let audio_player = AudioPlayer::new()?;

    // Initialize player state
    let mut player_state = PlayerState::new(database, audio_player, config.clone());

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create async event channel
    let (tx, mut rx) = mpsc::channel(100);

    // Spawn event listener
    tokio::spawn(async move {
        loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Ok(evt) = event::read() {
                    if tx.send(evt).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    // Main loop
    let mut last_draw = Instant::now();
    let draw_interval = Duration::from_millis(50); // 20 FPS for smooth visualization

    loop {
        // Update visualizer data
        player_state.update_visualizer();

        // Optimized redraw - only when needed
        let needs_redraw = player_state.needs_redraw() || last_draw.elapsed() >= draw_interval;
        
        if needs_redraw {
            terminal.draw(|f| {
                UI::render(f, &mut player_state);
            })?;
            last_draw = Instant::now();
            player_state.clear_redraw_flag();
        }

        // Handle events
        match tokio::time::timeout(Duration::from_millis(16), rx.recv()).await {
            Ok(Some(Event::Key(key))) => {
                // Check configurable keybinds
                let handled = match key.code {
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.quit, c, key.modifiers) => {
                        break;
                    }
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.play_pause, c, key.modifiers) => {
                        player_state.toggle_playback();
                        true
                    }
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.next, c, key.modifiers) => {
                        player_state.next_track();
                        true
                    }
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.previous, c, key.modifiers) => {
                        player_state.previous_track();
                        true
                    }
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.shuffle, c, key.modifiers) => {
                        player_state.toggle_shuffle();
                        true
                    }
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.volume_up, c, key.modifiers) => {
                        player_state.increase_volume();
                        true
                    }
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.volume_down, c, key.modifiers) => {
                        player_state.decrease_volume();
                        true
                    }
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.seek_forward, c, key.modifiers) => {
                        player_state.seek_forward();
                        true
                    }
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.seek_backward, c, key.modifiers) => {
                        player_state.seek_backward();
                        true
                    }
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.help, c, key.modifiers) => {
                        player_state.toggle_help();
                        true
                    }
                    KeyCode::Char(c) if matches_keybind(&config.keybinds.clear, c, key.modifiers) => {
                        player_state.clear_queue();
                        true
                    }
                    KeyCode::Char('-') => {
                        player_state.decrease_volume();
                        true
                    }
                    KeyCode::Up => {
                        player_state.scroll_up();
                        true
                    }
                    KeyCode::Down => {
                        player_state.scroll_down();
                        true
                    }
                    KeyCode::Enter if config.keybinds.select == "enter" => {
                        player_state.play_selected();
                        true
                    }
                    _ => false
                };

                if handled {
                    player_state.mark_needs_redraw();
                }
            }
            _ => {}
        }

        // Auto-advance to next track when current finishes
        if player_state.should_advance() {
            player_state.next_track();
            player_state.mark_needs_redraw();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn matches_keybind(keybind: &str, c: char, _modifiers: KeyModifiers) -> bool {
    keybind.to_lowercase() == c.to_string().to_lowercase()
        || (keybind == "space" && c == ' ')
}
