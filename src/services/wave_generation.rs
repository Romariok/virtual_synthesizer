use std::f32::consts::PI;
use std::time::Duration;
use wavegen::{dc_bias, sawtooth, wf};

const NOTE_TIME: f32 = 0.4;
const NOTE_TIME_LANCER: f32 = 0.7;
pub const AMPLITUDE: f32 = 1500.0;
const SAMPLE_RATE: f32 = 44100.0;

pub fn sine_wave(frequency: f32) -> impl Fn(f32) -> f32 {
    move |t| (t * frequency * 2.0 * PI).sin() as f32
}

pub fn square_wave(frequency: f32) -> impl Fn(f32) -> f32 {
    move |t| {
        let sin_wave = sine_wave(frequency);
        if sin_wave(t).is_sign_positive() {
            1.0
        } else {
            -1.0
        }
    }
}

pub fn sawtooth_wave(frequency: f32) -> impl Fn(f32) -> f32 {
    move |t| {
        let t_factor = t * frequency;
        t_factor - t_factor.floor() - 0.5
    }
}

pub fn triangle_wave(frequency: f32) -> impl Fn(f32) -> f32 {
    move |t| {
        let sawtooth_wave = sawtooth_wave(frequency);
        (sawtooth_wave(t).abs() - 0.25) * 4.0
    }
}

pub fn tangent_wave(frequency: f32) -> impl Fn(f32) -> f32 {
    move |t| {
        (((t * frequency * PI) - 0.5).tan() / 4.0)
            .max(-1.0)
            .min(1.0)
    }
}

pub fn gen_lancer_wave(freq: f32) -> Vec<i32> {
    let waveform = wf!(
        i32,
        SAMPLE_RATE,
        sawtooth!(freq, AMPLITUDE),
        dc_bias!(AMPLITUDE)
    );
    waveform
        .iter()
        .take((NOTE_TIME_LANCER * (2000 as f32)) as usize)
        .collect()
}

// Генерируем синусоидальную волну
pub fn generate_wave(amplitude: f32, f: impl Fn(f32) -> f32) -> Vec<i32> {
    let num_samples = (NOTE_TIME * SAMPLE_RATE as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for sample_idx in 0..num_samples {
        let t = sample_idx as f32 / SAMPLE_RATE as f32;
        let sample_value = (AMPLITUDE * f(t)) as i32;

        let time_remaining = (num_samples - sample_idx) as f32 / SAMPLE_RATE as f32;

        if time_remaining < 0.06 {
            let fade_out_duration = Duration::from_millis(60);
            let fade_out_samples = (fade_out_duration.as_secs_f32() * SAMPLE_RATE as f32) as usize;
            let fade_out_idx = num_samples - sample_idx;
            let fade_out_amp = amplitude * (fade_out_idx as f32 / fade_out_samples as f32);
            let fade_out_value = (fade_out_amp * f(t)) as i32;
            samples.push(fade_out_value);
        } else {
            samples.push(sample_value);
        }
    }
    samples
}

// Сложение волн
pub fn add_waves(waves: Vec<Vec<i32>>) -> Vec<i32> {
    let wave_len = waves[0].len();
    assert!(waves.iter().all(|wave| wave.len() == wave_len));

    let mut result_wave = vec![0i32; wave_len];

    for wave in waves {
        for (result_sample, sample) in result_wave.iter_mut().zip(wave.iter()) {
            *result_sample += *sample;
        }
    }

    result_wave
}
