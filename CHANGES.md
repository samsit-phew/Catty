# Changes Summary: Real Audio Visualizer + TOML Config

## What Was Changed

### 1. **Real FFT-Based Audio Visualization** (replaced random visualization)
   - **File**: `src/visualizer.rs`
   - Implemented actual audio frequency analysis using Fast Fourier Transform (FFT)
   - Captures real audio samples and processes them into frequency spectrum
   - Maps frequency data to visual bars with logarithmic scaling
   - Configurable bar count and smoothing factor

### 2. **Audio Sample Capture**
   - **File**: `src/audio.rs`
   - Modified audio player to capture samples during playback
   - Converts audio samples to f32 format for FFT processing
   - Shares sample buffer with visualizer

### 3. **TOML Configuration System**
   - **File**: `src/config.rs` (NEW)
   - Full configuration support with TOML format
   - Auto-generates config file at `~/.config/catty/config.toml`
   - Three configuration sections:
     - **Colors**: foreground, background, accent, visualizer
     - **Keybinds**: all key mappings (quit, play/pause, next, etc.)
     - **Visualizer**: bar_count, smoothing

### 4. **Configurable Colors**
   - **File**: `src/ui.rs`
   - All UI elements now use colors from config
   - Support for named colors and hex color codes (#RRGGBB)
   - Applied to borders, text, and visualizer bars

### 5. **Configurable Keybinds**
   - **File**: `src/main.rs`
   - Keyboard shortcuts now read from config file
   - Users can customize all keybinds
   - Helper function to match keybinds

### 6. **Updated Dependencies**
   - **File**: `Cargo.toml`
   - Added `toml = "0.8"` for config parsing
   - Added `rustfft = "6.2"` for FFT processing
   - Added `cpal = "0.15"` for audio device access

### 7. **Integration**
   - **File**: `src/player.rs`
   - Player state now includes config
   - Visualizer updates with real audio data
   - Sample buffer synchronization between audio player and visualizer

## Key Features

✅ **Real-time audio visualization** like CAVA
✅ **TOML configuration file** with auto-generation
✅ **Customizable colors** (named + hex codes)
✅ **Customizable keybinds**
✅ **Configurable visualizer parameters**
✅ **FFT-based frequency analysis**
✅ **Smooth, responsive visualization**

## Config File Location

```
~/.config/catty/config.toml
```

## How It Works

1. Audio player captures samples during playback
2. Samples are converted to f32 and stored in buffer
3. Visualizer reads buffer and performs FFT
4. FFT output is mapped to frequency bars
5. Bars are smoothed based on config setting
6. UI renders bars using configured colors

## Documentation

- See `CONFIG.md` for detailed configuration guide
- Config file auto-generates on first run with sensible defaults
