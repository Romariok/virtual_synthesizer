


pub use crate::services::generation::{
    generate_sine_wave,
    add_waves
};


struct BasicPiano{
    C: Vec<i16>,
    D: Vec<i16>,
    E: Vec<i16>,
    F: Vec<i16>,
    G: Vec<i16>,
    A: Vec<i16>,
    B: Vec<i16>
}

// pub fn init() {

// }

pub const basicPiano: BasicPiano = BasicPiano {
    C: generate_sine_wave(261.63, 5.0, 100.0),
    D: generate_sine_wave(293.66, 5.0, 100.0),
    E: generate_sine_wave(329.63, 5.0, 100.0),
    F: generate_sine_wave(349.23, 5.0, 100.0),
    G: generate_sine_wave(392.00, 5.0, 100.0),
    A: generate_sine_wave(440.00, 5.0, 100.0),
    B: generate_sine_wave(493.88, 5.0, 100.0)
};

