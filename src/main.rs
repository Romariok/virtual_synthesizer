mod data;
mod services;
extern crate rodio;

use data::collection::BasicPiano;
use winit::{
    event::{Event, VirtualKeyCode, ElementState},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};
use std::{fs::File, sync::Arc};
use std::io::BufReader;

use std::time::Duration;
use std::vec::Vec;
use std::thread;
use std::collections::HashMap;

use rodio::OutputStream;
use rodio::OutputStreamHandle;

extern crate hound;

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

fn play_wav_file(file_name: &str, handle: OutputStreamHandle) {
    let file_path = format!("resources/basic/{}", file_name);
    let file = File::open(&file_path).expect("Failed to open file");
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    sink.append(source);
    println!("Playing {:?}", file_name);
    thread::sleep(Duration::from_secs(1));
    sink.stop();
}

fn main() {
    let (stream, handle) = OutputStream::try_default().unwrap();
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut wav_files: Vec<String> = Vec::new();

    // Create a basic piano
    {
        let basic: BasicPiano = BasicPiano::initBasic();

        for (key, value) in basic.as_map() {
            let mut writer = hound::WavWriter::create(format!("resources/basic/{}.wav", key), spec).unwrap();
            wav_files.push(format!("resources/basic/{}.wav", key));

            for sample in value {
                writer.write_sample(sample).unwrap();
            }
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut threads: HashMap<String, thread::JoinHandle<()>> = HashMap::new();

    // Spawn a new thread for each file
    for file_name in &wav_files {
        let file_name_clone = file_name.to_string();
        let handle_clone = handle.clone();
        let thread = thread::spawn(move || {
            play_wav_file(&file_name_clone, handle_clone);
        });

        threads.insert(file_name.to_string(), thread);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event,
                window_id,
            } => match event {
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        if let Some(keycode) = input.virtual_keycode {
                            println!("Key pressed: {:?}", keycode);
                            match keycode {
                                VirtualKeyCode::C | VirtualKeyCode::D | VirtualKeyCode::E
                                | VirtualKeyCode::F | VirtualKeyCode::G | VirtualKeyCode::A
                                | VirtualKeyCode::B => {
                                    let path = format!("{}.wav", virtual_keycode_to_string(keycode));
                                    if let Some(thread_handle) = threads.get(&path) {
                                        let _ = thread_handle.join();
                                    } else {
                                        println!("WAV file not found in the HashMap.");
                                    }
                                }
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
