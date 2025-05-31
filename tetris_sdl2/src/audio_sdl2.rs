use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecWAV};
use sdl2::AudioSubsystem;
use std::path::Path;
use std::time::Instant;

pub struct AudioManager {
    device: Option<AudioDevice<WavAudio>>,
}

struct WavAudio {
    data: Vec<u8>,
    position: usize,
}

impl AudioCallback for WavAudio {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        let len = out.len();

        if self.position < self.data.len() {
            let remaining = self.data.len() - self.position;
            let copy_len = len.min(remaining);

            // Use efficient memory operations
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.data.as_ptr().add(self.position),
                    out.as_mut_ptr(),
                    copy_len,
                );
            }
            self.position += copy_len;

            if copy_len < len {
                out[copy_len..].fill(0); // Fill remaining buffer with silence
            }
        } else {
            out.fill(0); // Fill entire buffer with silence if no data is left
        }
    }
}

impl AudioManager {
    pub fn new() -> Self {
        Self { device: None }
    }

    pub fn init(&mut self, audio_subsystem: &AudioSubsystem) -> Result<(), String> {
        let path = Path::new("assets/TetrisBg_conv.wav");

        let wav = AudioSpecWAV::load_wav(path)?;
        println!("WAV format: freq={} channels={} format={:?}",
			wav.freq,
			wav.channels,
			wav.format);
        // let desired_spec = sdl2::audio::AudioSpecDesired {
        //     freq: Some(wav.freq),
        //     channels: Some(wav.channels as u8),
        //     samples: None,
        // };
        let desired_spec = sdl2::audio::AudioSpecDesired {
            freq: Some(wav.freq),
            channels: Some(wav.channels as u8),
            samples: None, // Use default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, move |_spec| {
            WavAudio {
                data: wav.buffer().to_vec(),
                position: 0,
            }
        })?;

        self.device = Some(device);
        Ok(())
    }

    pub fn play(&mut self) {
        if let Some(device) = &self.device {
            device.resume();
        }
    }

    pub fn stop(&mut self) {
        if let Some(device) = &self.device {
            device.pause();
        }
    }
}
