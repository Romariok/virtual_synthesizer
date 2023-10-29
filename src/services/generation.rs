


// Генерируем синусоидальную волну 
pub fn generate_sine_wave(freq: f32, duration_secs: f32, amplitude: f32) -> Vec<i32> {
   let sample_rate = 44100; // Частота дискретизации
   let num_samples = (duration_secs * sample_rate as f32) as usize;
   let mut samples = Vec::with_capacity(num_samples);

   for sample_idx in 0..num_samples {
       let t = sample_idx as f32 / sample_rate as f32;
       let sample_value = (amplitude * f32::sin(2.0 * std::f32::consts::PI * freq * t)) as i32;
       samples.push(sample_value);
   }
   samples
}
// Сложение волн
pub fn add_waves(waves: &[Vec<i16>]) -> Vec<i16> {
   let wave_len = waves[0].len();
   assert!(waves.iter().all(|wave| wave.len()== wave_len));

   let mut result_wave = vec![0i16; wave_len];

   for wave in waves{
       for (result_sample, sample) in result_wave.iter_mut().zip(wave.iter()) {
           *result_sample += *sample;
       }
   }

   result_wave
}

