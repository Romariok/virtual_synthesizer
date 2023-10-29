
use std::collections::HashMap;

use winit::event::VirtualKeyCode;

pub use crate::services::generation::{
    generate_sine_wave,
    add_waves
};

const NOTE_TIME: f32 = 0.5;
const AMPLITUDE: f32 = 1500.0;

pub struct BasicPiano{
    pub C: Vec<i32>,
    pub Csharp: Vec<i32>,
    pub D: Vec<i32>,
    pub Dsharp: Vec<i32>,
    pub E: Vec<i32>,
    pub F: Vec<i32>,
    pub Fsharp: Vec<i32>,
    pub G: Vec<i32>,
    pub Gsharp: Vec<i32>,
    pub A: Vec<i32>,
    pub Asharp: Vec<i32>,
    pub B: Vec<i32>
}

pub struct BaseKeys{
    pub C: String,
    pub Csharp: String,
    pub D: String,
    pub Dsharp: String,
    pub E: String,
    pub F: String,
    pub Fsharp: String,
    pub G: String,
    pub Gsharp: String,
    pub A: String,
    pub Asharp: String,
    pub B: String
}





impl BasicPiano {
    pub fn init_basic() -> BasicPiano {
        BasicPiano{
        C: generate_sine_wave(261.63, NOTE_TIME, AMPLITUDE),
        Csharp: generate_sine_wave(277.18, NOTE_TIME, AMPLITUDE),
        D: generate_sine_wave(293.66,NOTE_TIME, AMPLITUDE),
        Dsharp: generate_sine_wave(311.13, NOTE_TIME, AMPLITUDE),
        E: generate_sine_wave(329.63, NOTE_TIME, AMPLITUDE),
        F: generate_sine_wave(349.23, NOTE_TIME, AMPLITUDE),
        Fsharp: generate_sine_wave(369.99, NOTE_TIME, AMPLITUDE),
        G: generate_sine_wave(392.00, NOTE_TIME, AMPLITUDE),
        Gsharp: generate_sine_wave(415.30, NOTE_TIME, AMPLITUDE),
        A: generate_sine_wave(440.00, NOTE_TIME, AMPLITUDE),
        Asharp: generate_sine_wave(466.16, NOTE_TIME, AMPLITUDE),
        B: generate_sine_wave(493.88, NOTE_TIME, AMPLITUDE)
    }
    }
    pub fn as_map(&self) -> HashMap<&str, Vec<i32>> {
        HashMap::from([("C", self.C.clone()), ("C#", self.Csharp.clone()), ("D#", self.Dsharp.clone()),
        ("D", self.D.clone()), ("E", self.E.clone()),
         ("F", self.F.clone()), ("F#", self.Fsharp.clone()), ("G", self.G.clone()), 
         ("G#", self.Gsharp.clone()), ("A", self.A.clone()), ("A#", self.Asharp.clone()), ("B", self.B.clone())])
    }
}

impl BaseKeys{
    pub fn to_string(&self, key_code: VirtualKeyCode) -> String {
        match(key_code){
            VirtualKeyCode::X => self.C.clone(),
            VirtualKeyCode::D => self.Csharp.clone(),
            VirtualKeyCode::C => self.D.clone(),
            VirtualKeyCode::F => self.Dsharp.clone(),
            VirtualKeyCode::V => self.E.clone(),
            VirtualKeyCode::B => self.F.clone(),
            VirtualKeyCode::H => self.Fsharp.clone(),
            VirtualKeyCode::N => self.G.clone(),
            VirtualKeyCode::J => self.Gsharp.clone(),
            VirtualKeyCode::M => self.A.clone(),
            VirtualKeyCode::K => self.Asharp.clone(),
            VirtualKeyCode::Comma => self.B.clone(),
            _ => "Unknown".to_string(),
        }
    }
    pub fn init() -> BaseKeys {BaseKeys {
        C: "C".to_string(),
        Csharp: "C#".to_string(),
        D: "D".to_string(),
        Dsharp: "D#".to_string(),
        E: "E".to_string(),
        F: "F".to_string(),
        Fsharp: "F#".to_string(),
        G: "G".to_string(),
        Gsharp: "G#".to_string(),
        A: "A".to_string(),
        Asharp: "A#".to_string(),
        B: "B".to_string()
    }
}
}