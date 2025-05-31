use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct AudioManager {
    sink: Option<Sink>,
    _stream: Option<OutputStream>,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            sink: None,
            _stream: None,
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let path = Path::new("assets/TetrisBg.wav");
        let file = File::open(path).map_err(|e| format!("Failed to open WAV: {e}"))?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)
            .map_err(|e| format!("Failed to decode WAV: {e}"))?
            .repeat_infinite();

        let (stream, handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to open audio output stream: {e}"))?;
        let sink = Sink::try_new(&handle)
            .map_err(|e| format!("Failed to create audio sink: {e}"))?;

        sink.append(source);

        self.sink = Some(sink);
        self._stream = Some(stream);

        Ok(())
    }

    pub fn play(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }
}
