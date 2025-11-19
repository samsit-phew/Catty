use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::io::{BufReader, Cursor};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
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
        // Read file bytes into memory so we can create two independent decoders:
        // one for playback and one for extracting samples for the visualizer.
        let data = std::fs::read(path)?;

        // Playback decoder (convert to f32 samples)
        let playback_cursor = Cursor::new(data.clone());
        let playback_decoder = Decoder::new(BufReader::new(playback_cursor))?.convert_samples::<f32>();

        // Visualization decoder (separate reader so we don't consume playback samples)
        let vis_cursor = Cursor::new(data);
        let mut vis_decoder = Decoder::new(BufReader::new(vis_cursor))?.convert_samples::<f32>();

        // Store duration if available (from playback decoder)
        // Note: convert_samples() returns an adapter that still exposes total_duration()
        let duration = playback_decoder.total_duration();
        *self.current_duration.lock().unwrap() = duration;

        // Stop previous sink and replace with a new one for playback
        let sink = self.sink.lock().unwrap();
        sink.stop();
        drop(sink);

        let new_sink = Sink::try_new(&self.stream_handle)?;
        new_sink.append(playback_decoder);
        new_sink.play();
        *self.sink.lock().unwrap() = new_sink;

        // Spawn a background thread to consume the visualization decoder at roughly
        // the audio playback rate and push mono f32 samples into sample_buffer.
        let sample_buffer = Arc::clone(&self.sample_buffer);
        thread::spawn(move || {
            let channels = vis_decoder.channels() as usize;
            let sample_rate = vis_decoder.sample_rate();

            // We'll read in small chunks and sleep to approximate real-time
            let chunk_frames = 1024usize; // frames per chunk (per-channel frames)
            loop {
                // Collect up to chunk_frames * channels samples
                let mut tmp = Vec::with_capacity(chunk_frames * channels);
                for _ in 0..(chunk_frames * channels) {
                    if let Some(s) = vis_decoder.next() {
                        tmp.push(s);
                    } else {
                        break;
                    }
                }

                if tmp.is_empty() {
                    break; // finished
                }

                // Convert to mono by averaging channels if necessary
                if channels > 1 {
                    let frames = tmp.len() / channels;
                    let mut mono = Vec::with_capacity(frames);
                    for frame_idx in 0..frames {
                        let mut sum = 0.0f32;
                        for ch in 0..channels {
                            sum += tmp[frame_idx * channels + ch];
                        }
                        mono.push(sum / channels as f32);
                    }

                    let mut buf = sample_buffer.lock().unwrap();
                    buf.extend_from_slice(&mono);
                    if buf.len() > 8192 {
                        buf.drain(..4096);
                    }
                } else {
                    let mut buf = sample_buffer.lock().unwrap();
                    buf.extend_from_slice(&tmp);
                    if buf.len() > 8192 {
                        buf.drain(..4096);
                    }
                }

                // Sleep for approximately chunk_frames / sample_rate seconds
                if sample_rate > 0 {
                    let secs = (chunk_frames as f32) / (sample_rate as f32);
                    thread::sleep(Duration::from_secs_f32(secs));
                } else {
                    // fallback small sleep
                    thread::sleep(Duration::from_millis(10));
                }
            }
        });

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
