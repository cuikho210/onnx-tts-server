use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

pub struct OutputStreamWrapper {
    pub stream: OutputStream,
}
unsafe impl Send for OutputStreamWrapper {}
unsafe impl Sync for OutputStreamWrapper {}

pub struct Speaker {
    pub stream: OutputStreamWrapper,
    pub sink: Sink,
}
impl Default for Speaker {
    fn default() -> Self {
        Self::new()
    }
}

impl Speaker {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let stream = OutputStreamWrapper { stream };

        Self { stream, sink }
    }

    pub fn append_samples(&self, samples: Vec<f32>, sample_rate: u32) {
        let source = SamplesBuffer::new(1, sample_rate, samples);
        self.sink.append(source);
    }

    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end();
    }

    pub fn clear(&self) {
        self.sink.clear();
    }

    pub fn play(&self) {
        self.sink.play();
    }
}
