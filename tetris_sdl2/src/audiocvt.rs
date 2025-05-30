use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecWAV, AudioCVT};
use sdl2::AudioSubsystem;
use std::path::Path;

pub struct AudioManager {
    device: Option<AudioDevice<ConvertedAudio>>, // Use converted audio
}

struct ConvertedAudio {
    data: Vec<u8>,
    position: usize,
}

impl AudioCallback for ConvertedAudio {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        let len = out.len();

        if self.position + len <= self.data.len() {
            out.copy_from_slice(&self.data[self.position..self.position + len]);
            self.position += len;
        } else {
            let remaining = self.data.len() - self.position;
            out[..remaining].copy_from_slice(&self.data[self.position..]);
            out[remaining..].fill(0);
            self.position = self.data.len();
        }
    }
}

impl AudioManager {
    pub fn new() -> Self {
        Self { device: None }
    }

    pub fn init(&mut self, audio_subsystem: &AudioSubsystem) -> Result<(), String> {
        let path = Path::new("assets/TetrisBg.wav");

        let wav = AudioSpecWAV::load_wav(path)?;
        let desired_spec = audio_subsystem.default_audio_spec()?;

        // Create a converter from WAV spec to the device spec
        let mut cvt = AudioCVT::new(
            wav.format(),
            wav.channels(),
            wav.freq(),
            desired_spec.format,
            desired_spec.channels,
            desired_spec.freq,
        )?;

        // Convert the data
        cvt.convert(&wav.buffer());

        let device = audio_subsystem.open_playback(None, &desired_spec, move |_spec| {
            ConvertedAudio {
                data: cvt.buffer().to_vec(),
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
