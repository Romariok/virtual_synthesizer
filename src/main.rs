mod data;
mod services;

extern crate rodio;

use services::{sound_generation::play_sound, generate_flacs};
use data::collection::BasicPiano;
use std::sync::Arc;
use winit::{
    event::{ElementState, Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use async_std::task;
use rodio::OutputStream;




fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let (stream, handle) = OutputStream::try_default().unwrap();
    let styles = vec!(BasicPiano::init_basic(),  BasicPiano::init_lancer());

    generate_flacs(styles);

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
                            | VirtualKeyCode::I  => {
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

