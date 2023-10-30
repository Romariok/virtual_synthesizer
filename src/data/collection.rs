
use std::collections::HashMap;

use winit::event::VirtualKeyCode;

pub use crate::services::wave_generation::{
    generate_sine_wave,
    add_waves,
    gen_lancer_wave
};



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
    pub B: Vec<i32>,
    pub C2: Vec<i32>,
    pub Csharp2: Vec<i32>,
    pub D2: Vec<i32>,
    pub Dsharp2: Vec<i32>,
    pub E2: Vec<i32>,
    pub F2: Vec<i32>,
    pub Fsharp2: Vec<i32>,
    pub G2: Vec<i32>,
    pub Gsharp2: Vec<i32>,
    pub A2: Vec<i32>,
    pub Asharp2: Vec<i32>,
    pub B2: Vec<i32>,
    pub name: String
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
    pub B: String,
    pub C2: String,
    pub Csharp2: String,
    pub D2: String,
    pub Dsharp2: String,
    pub E2: String,
    pub F2: String,
    pub Fsharp2: String,
    pub G2: String,
    pub Gsharp2: String,
    pub A2: String,
    pub Asharp2: String,
    pub B2: String
}


 
impl BasicPiano {
    pub fn init_basic() -> BasicPiano {
        BasicPiano{
        C: generate_sine_wave(261.63),
        Csharp: generate_sine_wave(277.18),
        D: generate_sine_wave(293.66),
        Dsharp: generate_sine_wave(311.13),
        E: generate_sine_wave(329.63),
        F: generate_sine_wave(349.23),
        Fsharp: generate_sine_wave(369.99),
        G: generate_sine_wave(392.00),
        Gsharp: generate_sine_wave(415.30),
        A: generate_sine_wave(440.00),
        Asharp: generate_sine_wave(466.16),
        B: generate_sine_wave(493.88),
        C2: generate_sine_wave(523.25),
        Csharp2: generate_sine_wave(554.36),
        D2: generate_sine_wave(587.32),
        Dsharp2: generate_sine_wave(622.26),
        E2: generate_sine_wave(659.26),
        F2: generate_sine_wave(698.46),
        Fsharp2: generate_sine_wave(739.98),
        G2: generate_sine_wave(784.),
        Gsharp2: generate_sine_wave(830.60),
        A2: generate_sine_wave(880.),
        Asharp2: generate_sine_wave(923.32),
        B2: generate_sine_wave(987.75),
        name: "basic".to_string()
    }
    }

    pub fn init_lancer() -> BasicPiano{
        BasicPiano{
        C: gen_lancer_wave(261.63),
        Csharp: gen_lancer_wave(277.18),
        D: gen_lancer_wave(293.66),
        Dsharp: gen_lancer_wave(311.13),
        E: gen_lancer_wave(329.63),
        F: gen_lancer_wave(349.23),
        Fsharp: gen_lancer_wave(369.99),
        G: gen_lancer_wave(392.00),
        Gsharp: gen_lancer_wave(415.30),
        A: gen_lancer_wave(440.00),
        Asharp: gen_lancer_wave(466.16),
        B: gen_lancer_wave(493.88),
        C2: gen_lancer_wave(523.25),
        Csharp2: gen_lancer_wave(554.36),
        D2: gen_lancer_wave(587.32),
        Dsharp2: gen_lancer_wave(622.26),
        E2: gen_lancer_wave(659.26),
        F2: gen_lancer_wave(698.46),
        Fsharp2: gen_lancer_wave(739.98),
        G2: gen_lancer_wave(784.),
        Gsharp2: gen_lancer_wave(830.60),
        A2: gen_lancer_wave(880.),
        Asharp2: gen_lancer_wave(923.32),
        B2: gen_lancer_wave(987.75),
        name: "lancer".to_string()
    }
    }

    pub fn as_map(&self) -> HashMap<&str, Vec<i32>> {
        HashMap::from([("C", self.C.clone()), ("C#", self.Csharp.clone()), ("D#", self.Dsharp.clone()),
        ("D", self.D.clone()), ("E", self.E.clone()),
         ("F", self.F.clone()), ("F#", self.Fsharp.clone()), ("G", self.G.clone()), 
         ("G#", self.Gsharp.clone()), ("A", self.A.clone()), ("A#", self.Asharp.clone()), 
         ("B", self.B.clone()), ("C2", self.C2.clone()), ("C#2", self.Csharp2.clone()), ("D2", self.D2.clone()),
         ("D#2", self.Dsharp2.clone()), ("E2", self.E2.clone()), ("F2", self.F2.clone()), ("F#2", self.Fsharp2.clone()), 
         ("G2", self.G2.clone()), ("G#2", self.Gsharp2.clone()), ("A2", self.A2.clone()), ("A#2", self.Asharp2.clone()), 
         ("B2", self.B2.clone())])
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
            VirtualKeyCode::W => self.C2.clone(),
            VirtualKeyCode::Key3 => self.Csharp2.clone(),
            VirtualKeyCode::E => self.D2.clone(),
            VirtualKeyCode::Key4 => self.Dsharp2.clone(),
            VirtualKeyCode::R => self.E2.clone(),
            VirtualKeyCode::T => self.F2.clone(),
            VirtualKeyCode::Key6 => self.Fsharp2.clone(),
            VirtualKeyCode::Y => self.G2.clone(),
            VirtualKeyCode::Key7 => self.Gsharp2.clone(),
            VirtualKeyCode::U => self.A2.clone(),
            VirtualKeyCode::Key8 => self.Asharp2.clone(),
            VirtualKeyCode::I => self.B2.clone(),
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
        B: "B".to_string(),
        C2: "C2".to_string(),
        Csharp2: "C#2".to_string(),
        D2: "D2".to_string(),
        Dsharp2: "D#2".to_string(),
        E2: "E2".to_string(),
        F2: "F2".to_string(),
        Fsharp2: "F#2".to_string(),
        G2: "G2".to_string(),
        Gsharp2: "G#2".to_string(),
        A2: "A2".to_string(),
        Asharp2: "A#2".to_string(),
        B2: "B2".to_string(),
    }
}
}

pub fn virtual_keycode_to_string(key_code: VirtualKeyCode) -> String {
    match key_code {
        VirtualKeyCode::X
        | VirtualKeyCode::D
        | VirtualKeyCode::F
        | VirtualKeyCode::C
        | VirtualKeyCode::V
        | VirtualKeyCode::B
        | VirtualKeyCode::H
        | VirtualKeyCode::N
        | VirtualKeyCode::J
        | VirtualKeyCode::M
        | VirtualKeyCode::K
        | VirtualKeyCode::Comma                            
        | VirtualKeyCode::W 
        | VirtualKeyCode::Key3 
        | VirtualKeyCode::E 
        | VirtualKeyCode::Key4 
        | VirtualKeyCode::R 
        | VirtualKeyCode::T 
        | VirtualKeyCode::Key6 
        | VirtualKeyCode::Y 
        | VirtualKeyCode::Key7 
        | VirtualKeyCode::U 
        | VirtualKeyCode::Key8 
        | VirtualKeyCode::I => BaseKeys::init().to_string(key_code),
        _ => "Unknown".to_string(),
    }
}


