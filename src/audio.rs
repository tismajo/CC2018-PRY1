use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub struct Audio {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    bgm_sink: Sink,
    sfx_volume: f32,
}

impl Audio {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().expect("No hay dispositivo de audio");
        let bgm_sink = Sink::try_new(&handle).unwrap();

        // Música de fondo en loop
        if let Ok(file) = File::open("assets/music_background.ogg") {
            let src = rodio::Decoder::new(BufReader::new(file)).unwrap();
            bgm_sink.append(src.repeat_infinite());
        }

        bgm_sink.set_volume(0.7);
        bgm_sink.play();

        Self {
            _stream: stream,
            handle,
            bgm_sink,
            sfx_volume: 0.9,
        }
    }

    pub fn play_hit(&self) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            if let Ok(file) = File::open("assets/sfx_hit.wav") {
                let src = rodio::Decoder::new(BufReader::new(file)).unwrap();
                sink.append(src);
            }
            sink.set_volume(self.sfx_volume);
            sink.detach();
        }
    }
}
