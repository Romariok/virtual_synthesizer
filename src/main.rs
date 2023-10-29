mod data;
mod services;
extern crate hound;
extern crate rodio;


use data::collection::{BasicPiano, BaseKeys};
use hound::{SampleFormat};
use winit::{
    event::{ElementState, Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::{
    fs::{File, OpenOptions}, 
    io::{BufReader, Seek, Read, Write}, 
    time::Duration, vec::Vec, 
    sync::Arc, 
    thread, path::Path
};

use async_std::task;
use rodio::{OutputStream, OutputStreamHandle};

use flacenc::component::BitRepr;



fn virtual_keycode_to_string(key_code: VirtualKeyCode) -> String {
    match key_code {
        VirtualKeyCode::X | 
        VirtualKeyCode::D |
        VirtualKeyCode::F |
        VirtualKeyCode::C | 
        VirtualKeyCode::V | 
        VirtualKeyCode::B | 
        VirtualKeyCode::H |
        VirtualKeyCode::N |
        VirtualKeyCode::J | 
        VirtualKeyCode::M |
        VirtualKeyCode::K | 
        VirtualKeyCode::Comma=> BaseKeys::init().to_string(key_code),
        _ => "Unknown".to_string(),
    }
}


fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
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
        let input_file = format!("resources/basic/{}.flac", key);
        let (channels, bits_per_sample, sample_rate) = (2, 16, 44100);
        let config = flacenc::config::Encoder::default();
        let source = flacenc::source::MemSource::from_samples(
            value.as_slice(), channels, bits_per_sample, sample_rate);
        let flac_stream = flacenc::encode_with_fixed_block_size(
            &config, source, config.block_sizes[0]
        ).expect("Encode failed.");
        let mut sink = flacenc::bitsink::ByteSink::new();
        flac_stream.write(&mut sink);

        std::fs::write(input_file, sink.as_slice());
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
                            VirtualKeyCode::X | 
                            VirtualKeyCode::D |
                            VirtualKeyCode::F |
                            VirtualKeyCode::C | 
                            VirtualKeyCode::V | 
                            VirtualKeyCode::B | 
                            VirtualKeyCode::H |
                            VirtualKeyCode::N |
                            VirtualKeyCode::J | 
                            VirtualKeyCode::M |
                            VirtualKeyCode::K | 
                            VirtualKeyCode::Comma => {
                                // TODO: To use thread-safe values, use Arc;
                                // To understand async-rust start from this video:
                                // https://www.youtube.com/watch?v=77aRH6YBKyY&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=26&pp=iAQB
                                let handle_clone = Arc::clone(&handle_arc);
                                let keycode = keycode;
                                task::spawn(async move {
                                    play_sound(handle_clone, keycode).await;
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
 
async fn play_sound(handle: Arc<OutputStreamHandle>, keycode: VirtualKeyCode) {
    let file_name = virtual_keycode_to_string(keycode) + ".flac";
    let file_path = format!("resources/basic/{}", file_name.clone());

    let file = File::open(Path::new(&file_path)).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();


    let sink = rodio::Sink::try_new(&handle).unwrap();
    sink.append(source);
    println!("Playing {:?}", file_name);
    sink.sleep_until_end();

}
