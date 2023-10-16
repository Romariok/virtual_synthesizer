mod data;
mod services;
extern crate hound;
extern crate rodio;

use data::collection::BasicPiano;
use hound::SampleFormat;
use std::fs::File;
use std::io::BufReader;
use winit::{
    event::{ElementState, Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::collections::HashMap;
use std::time::Duration;
use std::vec::Vec;
use std::{sync::Arc, thread};

use async_std::task;
use rodio::OutputStream;
use rodio::OutputStreamHandle;

fn virtual_keycode_to_string(key_code: VirtualKeyCode) -> String {
    // TODO: Implement enum with letters instead of hardcoding
    // try to use &str since it is more applicable
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
    let file = File::open(file_path).expect("Failed to open file");
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    sink.append(source);
    println!("Playing {:?}", file_name);
    thread::sleep(Duration::from_secs(1));
    sink.stop();
}

/// TODO: Reduce code in main, since it should only call external methods and define variables for them
/// Big part of code can (and should) be moved to structs (rust java-objects) and enums
/// Look up #[derive(Default)] trait with impl block to reduce boilerplate
fn main() {
    // TODO: ALl variables should be defined at begin of main function
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut threads: HashMap<String, thread::JoinHandle<()>> = HashMap::new();
    let (stream, handle) = OutputStream::try_default().unwrap();
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut wav_files: Vec<String> = Vec::new();
    let basic: BasicPiano = BasicPiano::init_basic();

    for (key, value) in basic.as_map() {
        // TODO: This method may panic due to .unwrap(). Method desc even says so
        // Use it only when no panic may occur
        // Not production-friendly
        let mut writer =
            hound::WavWriter::create(format!("resources/basic/{}.wav", key), spec).unwrap();
        wav_files.push(format!("resources/basic/{}.wav", key));

        for sample in value {
            writer.write_sample(sample).unwrap();
        }
    }

    // Spawn a new thread for each file
    // TODO: ChatGPT code moment. 
    // May fail, lol
    for file_name in &wav_files {
        let file_name_clone = file_name.to_string();
        let handle_clone = handle.clone();

        let thread = thread::spawn(move || {
            play_wav_file(&file_name_clone, handle_clone);
        });

        threads.insert(file_name.to_string(), thread);
    }

    // Create an Arc (atomic reference counter) for thread-safe access to handle
    let handle_arc = Arc::new(handle);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: winit::event::WindowEvent::KeyboardInput { input, .. },
                window_id,
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    if let ElementState::Pressed = input.state {
                        println!("Key pressed: {:?}", keycode);
                        match keycode {
                            VirtualKeyCode::C
                            | VirtualKeyCode::D
                            | VirtualKeyCode::E
                            | VirtualKeyCode::F
                            | VirtualKeyCode::G
                            | VirtualKeyCode::A
                            | VirtualKeyCode::B => {
                                // TODO: To use thread-safe values, use Arc;
                                // To understand async-rust start from this video:
                                // https://www.youtube.com/watch?v=77aRH6YBKyY&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=26&pp=iAQB
                                let handle_clone = Arc::clone(&handle_arc);
                                let keycode = keycode;
                                task::spawn(async move {
                                    play_sound(handle_clone, &keycode).await;
                                });
                            }
                            _ => (),
                        }
                    }
                }
            }
            Event::LoopDestroyed => (),
            _ => (),
        }
    });
}

/// TODO: Build your logic
async fn play_sound(handle: Arc<OutputStreamHandle>, keycode: &VirtualKeyCode) {
    // Implement your sound playback logic here
    // You can use the `handle` to play the appropriate sound based on the `keycode`.
    // For example, you can play the sound associated with the provided key.
    todo!()
}
