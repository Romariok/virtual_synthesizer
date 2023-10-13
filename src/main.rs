mod data;
mod services;


use data::collection::BasicPiano;
use winit::{
    event::{Event, VirtualKeyCode, ElementState, KeyboardInput},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

use rodio::source::Source;
use rodio::OutputStream;
use rodio::OutputStreamHandle;


extern crate hound; // Библиотека для записи аудиофайлов

// implementation Display for VirtualKeyCode
fn virtual_keycode_to_string(key_code: VirtualKeyCode) -> String {
    match key_code {
        VirtualKeyCode::A => "A".to_string(),
        VirtualKeyCode::B => "B".to_string(),
        VirtualKeyCode::C => "C".to_string(),
        VirtualKeyCode::D => "D".to_string(),
        VirtualKeyCode::E => "E".to_string(),
        VirtualKeyCode::F => "F".to_string(),
        VirtualKeyCode::G => "G".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn play_sound(keycode: VirtualKeyCode) {
    let file = File::open("resources/basic/".to_string()+&virtual_keycode_to_string(keycode)+".wav").expect("Failed to open file");
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    let (stream, handle) = OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    sink.append(source);

    sleep(Duration::from_secs(1));  
    sink.stop();
}

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
// Incilization of sounds for piano
    {
        let basic: BasicPiano = BasicPiano::initBasic();
        // let note_basic: Vec<Vec<i16>> = Vec::from([basic.A, basic.B, basic.C, basic.D, basic.E, basic.F, basic.G]);


        for (key, value) in basic.as_map() {
            let mut writer = hound::WavWriter::
            create("resources/basic/".to_string()+key+".wav", spec).unwrap();
            for sample in value{
                writer.write_sample(sample).unwrap();
            }

        }
    }

// Infinite loop of tracking keyboard events
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event,
                window_id,
            } => match event {
                // Handle keyboard input events
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        if let Some(keycode) = input.virtual_keycode {
                            println!("Key pressed: {:?}", keycode);
                            match keycode {
                                VirtualKeyCode::C  => play_sound(keycode),
                                VirtualKeyCode::D  => play_sound(keycode),
                                VirtualKeyCode::E  => play_sound(keycode),
                                VirtualKeyCode::F  => play_sound(keycode),
                                VirtualKeyCode::G  => play_sound(keycode),
                                VirtualKeyCode::A  => play_sound(keycode),
                                VirtualKeyCode::B  => play_sound(keycode),
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            },
            Event::LoopDestroyed => return,
            _ => (),
        }
    });




}
