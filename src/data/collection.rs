
use std::collections::HashMap;

pub use crate::services::generation::{
    generate_sine_wave,
    add_waves
};

/// You can safely delete this line
/// It is only used to show that the struct is exported correctly

#[derive(Debug)]
pub struct BasicPiano{
    pub C: Vec<i16>,
    pub D: Vec<i16>,
    pub E: Vec<i16>,
    pub F: Vec<i16>,
    pub G: Vec<i16>,
    pub A: Vec<i16>,
    pub B: Vec<i16>
}

impl BasicPiano {
    pub fn initBasic() -> BasicPiano {
        BasicPiano{
        C: generate_sine_wave(261.63, 1.0, 100.0),
        D: generate_sine_wave(293.66, 1.0, 100.0),
        E: generate_sine_wave(329.63, 1.0, 100.0),
        F: generate_sine_wave(349.23, 1.0, 100.0),
        G: generate_sine_wave(392.00, 1.0, 100.0),
        A: generate_sine_wave(440.00, 1.0, 100.0),
        B: generate_sine_wave(493.88, 1.0, 100.0)
    }
    }
    pub fn as_map(&self) -> HashMap<&str, Vec<i16>> {
        HashMap::from([("C", self.C.clone()), ("D", self.D.clone()), ("E", self.E.clone()),
         ("F", self.F.clone()), ("G", self.G.clone()), ("A", self.A.clone()), ("B", self.B.clone())])
    }
}
