
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
    pub C: VirtualKeyCode,
    pub D: VirtualKeyCode,
    pub E: VirtualKeyCode,
    pub F: VirtualKeyCode,
    pub G: VirtualKeyCode,
    pub A: VirtualKeyCode,
    pub B: VirtualKeyCode
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
    pub fn to_string(&self) -> String {
        if (self.C==VirtualKeyCode::X){
            return "C".to_string();
        } else if (self.D == VirtualKeyCode::C){
            return "D".to_string();
        } else if (self.E == VirtualKeyCode::V){
            return "E".to_string();
        } else if (self.F == VirtualKeyCode::B){
            return "F".to_string();
        } else if (self.G == VirtualKeyCode::N){
            return "G".to_string();
        } else if (self.A == VirtualKeyCode::M){
            return "A".to_string();
        } else if (self.B == VirtualKeyCode::Comma){
            return "B".to_string();
        } else {
            return "Unknown".to_string();
            
        }
    }
    pub fn init() -> BaseKeys {BaseKeys {
        C: VirtualKeyCode::X,
        D: VirtualKeyCode::C,
        E: VirtualKeyCode::V,
        F: VirtualKeyCode::B,
        G: VirtualKeyCode::N,
        A: VirtualKeyCode::M,
        B: VirtualKeyCode::Comma
    }
}
}