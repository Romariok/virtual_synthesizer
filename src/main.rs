extern crate hound; // Библиотека для записи аудиофайлов

// Генерируем синусоидальную волну 
fn generate_sine_wave(freq: f32, duration_secs: f32, amplitude: f32) -> Vec<i16> {
    let sample_rate = 44100; // Частота дискретизации 
    let num_samples = (duration_secs * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for sample_idx in 0..num_samples {
        let t = sample_idx as f32 / sample_rate as f32;
        let sample_value = (amplitude * f32::sin(2.0 * std::f32::consts::PI * freq * t)) as i16;
        samples.push(sample_value);
    }

    samples
}
// Сложение волн
fn add_waves(wave1: &[i16], wave2: &[i16]) -> Vec<i16> {
    assert_eq!(wave1.len(), wave2.len());

    let mut result_wave = Vec::with_capacity(wave1.len());

    for (sample1, sample2) in wave1.iter().zip(wave2.iter()) {
        let sum_sample = sample1 + sample2;
        result_wave.push(sum_sample);
    }

    result_wave
}
fn main() {
    let wave1 = generate_sine_wave(440.0, 2.0, 100.0); // Генерируем синусоидальную волну 440 Гц, длительностью 2 секунды, амплитудой 0.5
    let wave2 = generate_sine_wave(660.0, 2.0, 100.0);
    let combined_wave = add_waves(&wave1, &wave2);

     let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine_wave.wav", spec).unwrap();

    for sample in combined_wave {
        writer.write_sample(sample).unwrap();
    }
}