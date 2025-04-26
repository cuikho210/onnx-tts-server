use rodio::{buffer::SamplesBuffer, OutputStream, Sink};
use std::time::Duration;
use tokio::time::sleep;

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
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let stream = OutputStreamWrapper { stream };

        Self { stream, sink }
    }
}
impl Speaker {
    pub fn append_samples(&self, samples: Vec<f32>, sample_rate: u32) {
        let source = SamplesBuffer::new(1, sample_rate, samples);
        self.sink.append(source);
    }

    // Non-blocking implementation to allow tokio to interrupt
    pub async fn sleep_until_end(&self) {
        while !self.sink.is_paused() && !self.sink.empty() {
            sleep(Duration::from_millis(200)).await;
        }
    }
}
