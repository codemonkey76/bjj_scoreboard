use std::io::Cursor;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

pub struct Audio {
    sound: &'static [u8],
    stream_handle: Option<OutputStreamHandle>,
    stream: Option<OutputStream>,
    sink: Option<Sink>,
}

impl Audio {
    pub fn play_air_horn(&self) {
        if let Some(sink) = &self.sink {
            let source = Decoder::new_wav(Cursor::new(self.sound)).expect("Unable to decode WAV file");
            sink.append(source);
        }
    }
    pub fn init(&mut self) {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        self.sink = Some(Sink::try_new(&stream_handle).unwrap());
        self.stream = Some(stream);
        self.stream_handle = Some(stream_handle);
    }
}
impl Default for Audio {
    fn default() -> Self {
        Self {
            sound: include_bytes!("../../assets/sounds/air-horn.wav"),
            stream_handle: None,
            stream: None,
            sink: None,
        }
    }
}