mod collection;
mod services;

use crate::collection::basicPiano;
extern crate hound; // Библиотека для записи аудиофайлов

fn main() {

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine_wave.wav", spec).unwrap();

    for sample in basicPiano::C {
        writer.write_sample(sample).unwrap();
    }
}
