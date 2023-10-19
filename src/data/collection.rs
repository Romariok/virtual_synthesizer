
use std::collections::HashMap;

use winit::event::VirtualKeyCode;

pub use crate::services::generation::{
    generate_sine_wave,
    add_waves
};


pub struct BasicPiano{
    pub C: Vec<i16>,
    pub D: Vec<i16>,
    pub E: Vec<i16>,
    pub F: Vec<i16>,
    pub G: Vec<i16>,
    pub A: Vec<i16>,
    pub B: Vec<i16>
}

pub struct BaseKeys{
    pub C: String,
    pub D: String,
    pub E: String,
    pub F: String,
    pub G: String,
    pub A: String,
    pub B: String
}





impl BasicPiano {
    pub fn init_basic() -> BasicPiano {
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

impl BaseKeys{
    pub fn to_string(&self, key_code: VirtualKeyCode) -> String {
        match(key_code){
            VirtualKeyCode::X => self.C.clone(),
            VirtualKeyCode::C => self.D.clone(),
            VirtualKeyCode::V => self.E.clone(),
            VirtualKeyCode::B => self.F.clone(),
            VirtualKeyCode::N => self.G.clone(),
            VirtualKeyCode::M => self.A.clone(),
            VirtualKeyCode::Comma => self.B.clone(),
            _ => "Unknown".to_string(),
        }
    }
    pub fn init() -> BaseKeys {BaseKeys {
        C: "C".to_string(),
        D: "D".to_string(),
        E: "E".to_string(),
        F: "F".to_string(),
        G: "G".to_string(),
        A: "A".to_string(),
        B: "B".to_string()
    }
}
}