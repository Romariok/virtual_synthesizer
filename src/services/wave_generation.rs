use std::time::Duration;
use wavegen::{wf, sine, square, sawtooth, dc_bias};

const NOTE_TIME: f32 = 0.4;
const NOTE_TIME_LANCER: f32 = 0.7;
const AMPLITUDE: f32 = 1500.0;
const SAMPLE_RATE: f32 = 44100.0;

pub fn gen_lancer_wave(freq: f32)-> Vec<i32> {
    let waveform = wf!(i32, SAMPLE_RATE, sawtooth!(freq, AMPLITUDE), dc_bias!(AMPLITUDE));
    waveform.iter().take((NOTE_TIME_LANCER*(2000 as f32)) as usize).collect()
}

pub fn gen_mario_wave(freq: f32)-> Vec<i32> {
    let waveform = wf!(i32, SAMPLE_RATE, sine!(freq, AMPLITUDE),square!(freq, AMPLITUDE));

    waveform.iter().take((NOTE_TIME*(2000 as f32)) as usize).collect()
}

// Генерируем синусоидальную волну 
pub fn generate_sine_wave(freq: f32) -> Vec<i32> {
    let num_samples = (NOTE_TIME * SAMPLE_RATE as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples);
 
    for sample_idx in 0..num_samples {
        let t = sample_idx as f32 / SAMPLE_RATE as f32;
        let sample_value = (AMPLITUDE * f32::sin(2.0 * std::f32::consts::PI * freq * t)) as i32;
 
        // Calculate the time remaining until the end of the sample
        let time_remaining = (num_samples - sample_idx) as f32 / SAMPLE_RATE as f32;
 
        // Apply a fade-out effect if there is less than 10 milliseconds remaining
        if time_remaining < 0.06 {
            let fade_out_duration = Duration::from_millis(60);
            let fade_out_samples = (fade_out_duration.as_secs_f32() * SAMPLE_RATE as f32) as usize;
            let fade_out_idx = num_samples - sample_idx;
            let fade_out_amp = AMPLITUDE * (fade_out_idx as f32 / fade_out_samples as f32);
            let fade_out_value = (fade_out_amp * f32::sin(2.0 * std::f32::consts::PI * freq * t)) as i32;
            samples.push(fade_out_value);
        } else {
            samples.push(sample_value);
        }
    }
    samples
 }
// Сложение волн
pub fn add_waves(waves: &[Vec<i32>]) -> Vec<i32> {
   let wave_len = waves[0].len();
   assert!(waves.iter().all(|wave| wave.len()== wave_len));

   let mut result_wave = vec![0i32; wave_len];

   for wave in waves{
       for (result_sample, sample) in result_wave.iter_mut().zip(wave.iter()) {
           *result_sample += *sample;
       }
   }

   result_wave
}

