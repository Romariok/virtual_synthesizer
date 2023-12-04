use std::f32::consts::PI;
use std::time::Duration;
use wavegen::{dc_bias, sawtooth, wf};
use crate::services::filter::envelope;

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

pub fn organ(frequency: f32) -> impl Fn(f32) -> f32 {
    move |t| {
        let frequency_2 = (frequency / 2.0) * 3.0;
        sine_wave(frequency)(t) + 0.2 * sine_wave(frequency_2)(t)
    }
}

pub fn bell(frequency: f32) -> impl Fn(f32) -> f32 {
    move |t| {
        // Frequency, amplitude, decay
        let harmonics_table: [(f32, f32, f32); 9] = [
            (0.56, 1.5, 1.0),
            (0.92, 0.5, 2.0),
            (1.19, 0.25, 4.0),
            (1.71, 0.125, 6.0),
            (2.00, 0.062_5, 8.4),
            (2.74, 0.031_25, 10.8),
            (3.00, 0.015_625, 13.6),
            (3.76, 0.007_812_5, 16.4),
            (4.07, 0.003_906_25, 19.6),
        ];

        harmonics_table.iter().fold(0.0, |acc, h| {
            acc + sine_wave(frequency * h.0)(t) *h.1* envelope(t, 0., 0.9 * h.2)
        }) / 2.0
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
        let sample_value = (amplitude * f(t)* envelope(t, 0., 0.42 )) as i32;
        samples.push(sample_value);

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
