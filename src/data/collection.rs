use std::collections::HashMap;

use winit::event::VirtualKeyCode;


pub use crate::services::wave_generation::{
    generate_wave,
    // add_waves,
    sine_wave,
    // square_wave,
    // sawtooth_wave,
    // triangle_wave,
    // tangent_wave,
    bell,
    organ,
    gen_lancer_wave,
    AMPLITUDE
};


#[derive(Clone)]
pub struct BasicPiano{
    pub c: Vec<i32>,
    pub csharp: Vec<i32>,
    pub d: Vec<i32>,
    pub dsharp: Vec<i32>,
    pub e: Vec<i32>,
    pub f: Vec<i32>,
    pub fsharp: Vec<i32>,
    pub g: Vec<i32>,
    pub gsharp: Vec<i32>,
    pub a: Vec<i32>,
    pub asharp: Vec<i32>,
    pub b: Vec<i32>,
    pub c2: Vec<i32>,
    pub csharp2: Vec<i32>,
    pub d2: Vec<i32>,
    pub dsharp2: Vec<i32>,
    pub e2: Vec<i32>,
    pub f2: Vec<i32>,
    pub fsharp2: Vec<i32>,
    pub g2: Vec<i32>,
    pub gsharp2: Vec<i32>,
    pub a2: Vec<i32>,
    pub asharp2: Vec<i32>,
    pub b2: Vec<i32>,
    pub name: String
}


pub struct BaseKeys{
    pub c: String,
    pub csharp: String,
    pub d: String,
    pub dsharp: String,
    pub e: String,
    pub f: String,
    pub fsharp: String,
    pub g: String,
    pub gsharp: String,
    pub a: String,
    pub asharp: String,
    pub b: String,
    pub c2: String,
    pub csharp2: String,
    pub d2: String,
    pub dsharp2: String,
    pub e2: String,
    pub f2: String,
    pub fsharp2: String,
    pub g2: String,
    pub gsharp2: String,
    pub a2: String,
    pub asharp2: String,
    pub b2: String
}


 
impl BasicPiano {
    pub fn init_sine() -> BasicPiano {
        BasicPiano{
        c: generate_wave(AMPLITUDE, sine_wave(261.63)),
        csharp: generate_wave(AMPLITUDE, sine_wave(277.18)),
        d: generate_wave(AMPLITUDE, sine_wave(293.66)),
        dsharp: generate_wave(AMPLITUDE, sine_wave(311.13)),
        e: generate_wave(AMPLITUDE, sine_wave(329.63)),
        f: generate_wave(AMPLITUDE, sine_wave(349.23)),
        fsharp: generate_wave(AMPLITUDE, sine_wave(369.99)),
        g: generate_wave(AMPLITUDE, sine_wave(392.00)),
        gsharp: generate_wave(AMPLITUDE, sine_wave(415.30)),
        a: generate_wave(AMPLITUDE, sine_wave(440.00)),
        asharp: generate_wave(AMPLITUDE, sine_wave(466.16)),
        b: generate_wave(AMPLITUDE, sine_wave(493.88)),
        c2: generate_wave(AMPLITUDE, sine_wave(523.25)),
        csharp2: generate_wave(AMPLITUDE, sine_wave(554.36)),
        d2: generate_wave(AMPLITUDE, sine_wave(587.32)),
        dsharp2: generate_wave(AMPLITUDE, sine_wave(622.25)),
        e2: generate_wave(AMPLITUDE, sine_wave(659.26)),
        f2: generate_wave(AMPLITUDE, sine_wave(698.46)),
        fsharp2: generate_wave(AMPLITUDE, sine_wave(739.98)),
        g2: generate_wave(AMPLITUDE, sine_wave(784.)),
        gsharp2: generate_wave(AMPLITUDE, sine_wave(830.60)),
        a2: generate_wave(AMPLITUDE, sine_wave(880.)),
        asharp2: generate_wave(AMPLITUDE, sine_wave(923.32)),
        b2: generate_wave(AMPLITUDE, sine_wave(987.75)),
        name: "sine".to_string()
    }
    }

    pub fn init_lancer() -> BasicPiano{
        BasicPiano{
        c: gen_lancer_wave(261.63),
        csharp: gen_lancer_wave(277.18),
        d: gen_lancer_wave(293.66),
        dsharp: gen_lancer_wave(311.13),
        e: gen_lancer_wave(329.63),
        f: gen_lancer_wave(349.23),
        fsharp: gen_lancer_wave(369.99),
        g: gen_lancer_wave(392.00),
        gsharp: gen_lancer_wave(415.30),
        a: gen_lancer_wave(440.00),
        asharp: gen_lancer_wave(466.16),
        b: gen_lancer_wave(493.88),
        c2: gen_lancer_wave(523.25),
        csharp2: gen_lancer_wave(554.36),
        d2: gen_lancer_wave(587.32),
        dsharp2: gen_lancer_wave(622.26),
        e2: gen_lancer_wave(659.26),
        f2: gen_lancer_wave(698.46),
        fsharp2: gen_lancer_wave(739.98),
        g2: gen_lancer_wave(784.),
        gsharp2: gen_lancer_wave(830.60),
        a2: gen_lancer_wave(880.),
        asharp2: gen_lancer_wave(923.32),
        b2: gen_lancer_wave(987.75),
        name: "lancer".to_string()
    }
    }
    
    pub fn init_bell() -> BasicPiano {
        BasicPiano{
        c: generate_wave(AMPLITUDE, bell(261.63)),
        csharp: generate_wave(AMPLITUDE, bell(277.18)),
        d: generate_wave(AMPLITUDE, bell(293.66)),
        dsharp: generate_wave(AMPLITUDE, bell(311.13)),
        e: generate_wave(AMPLITUDE, bell(329.63)),
        f: generate_wave(AMPLITUDE, bell(349.23)),
        fsharp: generate_wave(AMPLITUDE, bell(369.99)),
        g: generate_wave(AMPLITUDE, bell(392.00)),
        gsharp: generate_wave(AMPLITUDE, bell(415.30)),
        a: generate_wave(AMPLITUDE, bell(440.00)),
        asharp: generate_wave(AMPLITUDE, bell(466.16)),
        b: generate_wave(AMPLITUDE, bell(493.88)),
        c2: generate_wave(AMPLITUDE, bell(523.25)),
        csharp2: generate_wave(AMPLITUDE, bell(554.36)),
        d2: generate_wave(AMPLITUDE, bell(587.32)),
        dsharp2: generate_wave(AMPLITUDE, bell(622.25)),
        e2: generate_wave(AMPLITUDE, bell(659.26)),
        f2: generate_wave(AMPLITUDE, bell(698.46)),
        fsharp2: generate_wave(AMPLITUDE, bell(739.98)),
        g2: generate_wave(AMPLITUDE, bell(784.)),
        gsharp2: generate_wave(AMPLITUDE, bell(830.60)),
        a2: generate_wave(AMPLITUDE, bell(880.)),
        asharp2: generate_wave(AMPLITUDE, bell(923.32)),
        b2: generate_wave(AMPLITUDE, bell(987.75)),
        name: "bell".to_string()
    }
    }

    pub fn init_organ() -> BasicPiano {
        BasicPiano{
        c: generate_wave(AMPLITUDE, organ(261.63)),
        csharp: generate_wave(AMPLITUDE, organ(277.18)),
        d: generate_wave(AMPLITUDE, organ(293.66)),
        dsharp: generate_wave(AMPLITUDE, organ(311.13)),
        e: generate_wave(AMPLITUDE, organ(329.63)),
        f: generate_wave(AMPLITUDE, organ(349.23)),
        fsharp: generate_wave(AMPLITUDE, organ(369.99)),
        g: generate_wave(AMPLITUDE, organ(392.00)),
        gsharp: generate_wave(AMPLITUDE, organ(415.30)),
        a: generate_wave(AMPLITUDE, organ(440.00)),
        asharp: generate_wave(AMPLITUDE, organ(466.16)),
        b: generate_wave(AMPLITUDE, organ(493.88)),
        c2: generate_wave(AMPLITUDE, organ(523.25)),
        csharp2: generate_wave(AMPLITUDE, organ(554.36)),
        d2: generate_wave(AMPLITUDE, organ(587.32)),
        dsharp2: generate_wave(AMPLITUDE, organ(622.25)),
        e2: generate_wave(AMPLITUDE, organ(659.26)),
        f2: generate_wave(AMPLITUDE, organ(698.46)),
        fsharp2: generate_wave(AMPLITUDE, organ(739.98)),
        g2: generate_wave(AMPLITUDE, organ(784.)),
        gsharp2: generate_wave(AMPLITUDE, organ(830.60)),
        a2: generate_wave(AMPLITUDE, organ(880.)),
        asharp2: generate_wave(AMPLITUDE, organ(923.32)),
        b2: generate_wave(AMPLITUDE, organ(987.75)),
        name: "organ".to_string()
    }
    }
    
    pub fn as_map(&self) -> HashMap<&str, Vec<i32>> {
        HashMap::from([("C", self.c.clone()), ("C#", self.csharp.clone()), ("D#", self.dsharp.clone()),
        ("D", self.d.clone()), ("E", self.e.clone()),
         ("F", self.f.clone()), ("F#", self.fsharp.clone()), ("G", self.g.clone()), 
         ("G#", self.gsharp.clone()), ("A", self.a.clone()), ("A#", self.asharp.clone()), 
         ("B", self.b.clone()), ("C2", self.c2.clone()), ("C#2", self.csharp2.clone()), ("D2", self.d2.clone()),
         ("D#2", self.dsharp2.clone()), ("E2", self.e2.clone()), ("F2", self.f2.clone()), ("F#2", self.fsharp2.clone()), 
         ("G2", self.g2.clone()), ("G#2", self.gsharp2.clone()), ("A2", self.a2.clone()), ("A#2", self.asharp2.clone()), 
         ("B2", self.b2.clone())])
    }
}

impl BaseKeys{
    pub fn to_string(&self, key_code: VirtualKeyCode) -> String {
        match key_code {
            VirtualKeyCode::X => self.c.clone(),
            VirtualKeyCode::D => self.csharp.clone(),
            VirtualKeyCode::C => self.d.clone(),
            VirtualKeyCode::F => self.dsharp.clone(),
            VirtualKeyCode::V => self.e.clone(),
            VirtualKeyCode::B => self.f.clone(),
            VirtualKeyCode::H => self.fsharp.clone(),
            VirtualKeyCode::N => self.g.clone(),
            VirtualKeyCode::J => self.gsharp.clone(),
            VirtualKeyCode::M => self.a.clone(),
            VirtualKeyCode::K => self.asharp.clone(),
            VirtualKeyCode::Comma => self.b.clone(),
            VirtualKeyCode::W => self.c2.clone(),
            VirtualKeyCode::Key3 => self.csharp2.clone(),
            VirtualKeyCode::E => self.d2.clone(),
            VirtualKeyCode::Key4 => self.dsharp2.clone(),
            VirtualKeyCode::R => self.e2.clone(),
            VirtualKeyCode::T => self.f2.clone(),
            VirtualKeyCode::Key6 => self.fsharp2.clone(),
            VirtualKeyCode::Y => self.g2.clone(),
            VirtualKeyCode::Key7 => self.gsharp2.clone(),
            VirtualKeyCode::U => self.a2.clone(),
            VirtualKeyCode::Key8 => self.asharp2.clone(),
            VirtualKeyCode::I => self.b2.clone(),
            _ => "Unknown".to_string(),
        }
    }
    pub fn init() -> BaseKeys {BaseKeys {
        c: "C".to_string(),
        csharp: "C#".to_string(),
        d: "D".to_string(),
        dsharp: "D#".to_string(),
        e: "E".to_string(),
        f: "F".to_string(),
        fsharp: "F#".to_string(),
        g: "G".to_string(),
        gsharp: "G#".to_string(),
        a: "A".to_string(),
        asharp: "A#".to_string(),
        b: "B".to_string(),
        c2: "C2".to_string(),
        csharp2: "C#2".to_string(),
        d2: "D2".to_string(),
        dsharp2: "D#2".to_string(),
        e2: "E2".to_string(),
        f2: "F2".to_string(),
        fsharp2: "F#2".to_string(),
        g2: "G2".to_string(),
        gsharp2: "G#2".to_string(),
        a2: "A2".to_string(),
        asharp2: "A#2".to_string(),
        b2: "B2".to_string(),
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


