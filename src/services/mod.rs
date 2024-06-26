pub mod wave_generation;
pub mod sound_generation;
pub mod filter;

pub use crate::services::wave_generation::{AMPLITUDE, sine_wave
   // , square_wave
};
pub use crate::services::sound_generation::play_sound;
pub use crate::services::sound_generation::generate_flacs;
pub use crate::services::filter::envelope;