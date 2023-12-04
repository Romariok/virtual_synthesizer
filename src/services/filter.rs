pub fn envelope(relative_t: f32, attack: f32, decay: f32) -> f32 {
   if relative_t < 0.0 {
       return 0.0;
   } else if relative_t < attack {
       return relative_t / attack;
   } else if relative_t < attack + decay {
       return 1.0 - (relative_t - attack) / decay;
   }

   0.0
}