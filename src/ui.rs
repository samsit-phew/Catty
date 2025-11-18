use crate::config::Config;
use crate::player::PlayerState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};

/// UI rendering
pub struct UI;

impl UI {
    /// Render the entire UI
    pub fn render(f: &mut Frame, state: &mut PlayerState) {
        let size = f.area();

        // Main layout: vertical split
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),      // Title bar
                Constraint::Percentage(30), // Track list (30%)
                Constraint::Percentage(60), // Visualizer (60% - MUCH BIGGER!)
                Constraint::Length(5),      // Controls
            ])
            .split(size);

        // Render title
        Self::render_title(f, chunks[0], &state.config);

        // Render track list
        Self::render_track_list(f, chunks[1], state);

        // Render visualizer
        Self::render_visualizer(f, chunks[2], state);

        // Render controls
        Self::render_controls(f, chunks[3], state);
    }

    /// Render title bar
    fn render_title(f: &mut Frame, area: Rect, config: &Config) {
        let accent_color = Config::parse_color(&config.colors.accent);
        let title = Paragraph::new("🎵 Catty Music Player")
            .style(Style::default().fg(accent_color).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, area);
    }

    /// Render track list
    fn render_track_list(f: &mut Frame, area: Rect, state: &PlayerState) {
        let tracks = state.database.get_tracks();
        let foreground = Config::parse_color(&state.config.colors.foreground);
        let accent = Config::parse_color(&state.config.colors.accent);
        
        // Calculate visible range
        let visible_height = area.height.saturating_sub(2) as usize;
        let selected = state.list_state;
        
        // Adjust scroll to keep selection visible
        let scroll_offset = if selected < state.scroll_offset {
            selected
        } else if selected >= state.scroll_offset + visible_height {
            selected.saturating_sub(visible_height - 1)
        } else {
            state.scroll_offset
        };

        let items: Vec<ListItem> = tracks
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(visible_height)
            .map(|(i, track)| {
                let is_current = state.current_track_index == Some(i);
                let is_selected = i == selected;
                
                let prefix = if is_current {
                    if state.is_playing { "▶ " } else { "⏸ " }
                } else {
                    "  "
                };

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else if is_current {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(foreground)
                };

                let content = format!("{}{}", prefix, track.title);
                ListItem::new(content).style(style)
            })
            .collect();

        let title = format!(" Tracks ({}/{}) ", selected + 1, tracks.len());
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title).border_style(Style::default().fg(accent)));

        f.render_widget(list, area);
    }

    /// Render CAVA-style visualizer
    fn render_visualizer(f: &mut Frame, area: Rect, state: &PlayerState) {
        let bars = state.visualizer.get_bars();
        let viz_fg = Config::parse_color(&state.config.colors.visualizer_foreground);
        let viz_bg = Config::parse_color(&state.config.colors.visualizer_background);
        let accent = Config::parse_color(&state.config.colors.accent);
        
        let width = area.width.saturating_sub(2) as usize;
        let height = area.height.saturating_sub(2) as usize;
        
        // Adjust bar count to fit width
        let bar_count = width.min(bars.len());
        let bars_to_show = &bars[..bar_count];

        // Create multi-line bar visualization (vertical bars)
        let mut lines: Vec<String> = vec![String::new(); height];
        
        for &bar_height in bars_to_show.iter() {
            let filled_rows = (bar_height * height as f32) as usize;
            
            for row in 0..height {
                let inverted_row = height - 1 - row; // Draw from bottom to top
                
                if inverted_row < filled_rows {
                    // Filled part - use foreground color blocks
                    lines[row].push('█');
                } else {
                    // Empty part - use background
                    lines[row].push(' ');
                }
            }
        }

        // Create spans with colors
        let styled_lines: Vec<Line> = lines.into_iter().map(|line| {
            Line::from(Span::styled(line, Style::default().fg(viz_fg).bg(viz_bg)))
        }).collect();

        let visualizer = Paragraph::new(styled_lines)
            .block(Block::default().borders(Borders::ALL).title(" Visualizer ").border_style(Style::default().fg(accent)));

        f.render_widget(visualizer, area);
    }

    /// Render controls and status
    fn render_controls(f: &mut Frame, area: Rect, state: &PlayerState) {
        let accent = Config::parse_color(&state.config.colors.accent);
        let foreground = Config::parse_color(&state.config.colors.foreground);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Volume
                Constraint::Length(3), // Status and controls
            ])
            .split(area);

        // Volume gauge
        let volume_percent = (state.volume * 100.0) as u16;
        let volume_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(" Volume ").border_style(Style::default().fg(accent)))
            .gauge_style(Style::default().fg(accent))
            .percent(volume_percent);
        f.render_widget(volume_gauge, chunks[0]);

        // Status and controls
        let current_track = state
            .get_current_track()
            .map(|t| t.title.clone())
            .unwrap_or_else(|| "No track playing".to_string());

        let status = if state.is_playing { "Playing" } else { "Paused" };
        let shuffle_status = if state.shuffle { "ON" } else { "OFF" };
        
        let controls = vec![
            Line::from(vec![
                Span::styled("Now: ", Style::default().fg(Color::Gray)),
                Span::styled(current_track, Style::default().fg(foreground)),
            ]),
            Line::from(vec![
                Span::raw(format!("{}: Play/Pause | ", state.config.keybinds.play_pause)),
                Span::raw(format!("{}: Next | ", state.config.keybinds.next)),
                Span::raw(format!("{}: Prev | ", state.config.keybinds.previous)),
                Span::raw(format!("{}: Shuffle({}) | ", state.config.keybinds.shuffle, shuffle_status)),
                Span::raw(format!("{}/-: Vol | ", state.config.keybinds.volume_up)),
                Span::raw(format!("{}: Play | ", state.config.keybinds.select)),
                Span::raw(format!("{}: Clear | ", state.config.keybinds.clear)),
                Span::raw(format!("{}: Quit", state.config.keybinds.quit)),
            ]),
        ];

        let controls_widget = Paragraph::new(controls)
            .block(Block::default().borders(Borders::ALL).title(format!(" {} ", status)).border_style(Style::default().fg(accent)));

        f.render_widget(controls_widget, chunks[1]);
    }
}
