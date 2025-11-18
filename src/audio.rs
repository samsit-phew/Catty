use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Audio player using rodio with sample capturing for visualization
pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Arc<Mutex<Sink>>,
    current_duration: Arc<Mutex<Option<Duration>>>,
    sample_buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioPlayer {
    /// Create a new audio player
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        
        Ok(Self {
            _stream: stream,
            stream_handle,
            sink: Arc::new(Mutex::new(sink)),
            current_duration: Arc::new(Mutex::new(None)),
            sample_buffer: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Play a track from file path
    pub fn play(&self, path: &Path) -> Result<()> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;
        
        // Store duration if available
        let duration = source.total_duration();
        *self.current_duration.lock().unwrap() = duration;

        let sink = self.sink.lock().unwrap();
        sink.stop();
        drop(sink);

        // Create capturing source
        let sample_buffer = Arc::clone(&self.sample_buffer);
        let capturing_source = source.periodic_access(Duration::from_millis(10), move |src| {
            if let Some(sample) = src.current_frame_len() {
                // Capture mono samples for visualization
                let mut buffer = sample_buffer.lock().unwrap();
                
                // Convert to mono and store (simple averaging if stereo)
                for s in src.by_ref().take(sample.min(2048)) {
                    // Convert i16 to f32 and normalize to -1.0..1.0 range
                    buffer.push(s as f32 / i16::MAX as f32);
                    
                    // Keep buffer size manageable
                    if buffer.len() > 8192 {
                        buffer.drain(..4096);
                    }
                }
            }
        });

        // Create new sink for new track
        let new_sink = Sink::try_new(&self.stream_handle)?;
        new_sink.append(capturing_source);
        new_sink.play();
        
        *self.sink.lock().unwrap() = new_sink;

        Ok(())
    }

    /// Get sample buffer for visualization
    pub fn get_sample_buffer(&self) -> Arc<Mutex<Vec<f32>>> {
        Arc::clone(&self.sample_buffer)
    }

    /// Pause playback
    pub fn pause(&self) {
        self.sink.lock().unwrap().pause();
    }

    /// Resume playback
    pub fn resume(&self) {
        self.sink.lock().unwrap().play();
    }

    /// Stop playback
    pub fn stop(&self) {
        self.sink.lock().unwrap().stop();
        self.sample_buffer.lock().unwrap().clear();
    }

    /// Check if player is paused
    pub fn is_paused(&self) -> bool {
        self.sink.lock().unwrap().is_paused()
    }

    /// Check if player is empty (finished playing)
    pub fn is_empty(&self) -> bool {
        self.sink.lock().unwrap().empty()
    }

    /// Set volume (0.0 to 1.0)
    pub fn set_volume(&self, volume: f32) {
        self.sink.lock().unwrap().set_volume(volume);
    }

    /// Get current volume
    pub fn get_volume(&self) -> f32 {
        self.sink.lock().unwrap().volume()
    }

    /// Get current track duration
    pub fn get_duration(&self) -> Option<Duration> {
        *self.current_duration.lock().unwrap()
    }
}
