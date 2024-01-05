mod data;
mod services;
mod utils;

extern crate rodio;
extern crate winit;

use data::collection::BasicPiano;
use services::{generate_flacs, sound_generation::play_sound};
use softbuffer::{Context, Surface};
use std::{path::Path, sync::Arc};
use utils::util::{load_icon, draw_surface};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use async_std::task;
use rodio::OutputStream;

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/piano.jpg");
    let icon = load_icon(Path::new(path));

    let surface_width = 800.0;
    let surface_height = 600.0;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_theme(Some(winit::window::Theme::Dark))
        .with_title("Пианинка 😁")
        .with_window_icon(Some(icon))
        .with_inner_size(LogicalSize::new(surface_width, surface_height))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let context = unsafe { Context::new(&window) }.unwrap();
    let mut surface = unsafe { Surface::new(&context, &window) }.unwrap();

    let (_stream, handle) = OutputStream::try_default().unwrap();
    let styles = vec![
        BasicPiano::init_sine(),
        BasicPiano::init_lancer(),
        BasicPiano::init_organ(),
        BasicPiano::init_bell(),
    ];
    let styles_names = styles
        .iter()
        .map(|style| style.name.clone())
        .collect::<Vec<String>>();
    let mut index_style = 0;

    generate_flacs(styles);

    let handle_arc = Arc::new(handle);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let style_name = styles_names.clone();
        match event {
            Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                winit::event::WindowEvent::DroppedFile(path) => {
                    window.set_window_icon(Some(load_icon(&path)));
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
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
                                | VirtualKeyCode::I => {
                                    let handle_clone = Arc::clone(&handle_arc);
                                    let keycode = keycode;
                                    task::spawn(async move {
                                        play_sound(
                                            handle_clone,
                                            keycode,
                                            style_name[index_style].to_string(),
                                        )
                                        .await;
                                    });
                                }
                                VirtualKeyCode::Left => {
                                    if index_style > 0 {
                                        index_style -= 1;
                                    }
                                    println!("Tune: {}", style_name[index_style].to_string());
                                }
                                VirtualKeyCode::Right => {
                                    if index_style < styles_names.len() - 1 {
                                        index_style += 1;
                                    }
                                    println!("Tune: {}", style_name[index_style].to_string());
                                }
                                VirtualKeyCode::Escape => {
                                    *control_flow = ControlFlow::Exit;
                                }
                                _ => (),
                            }
                        }
                    }
                }

                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                let pixmap = draw_surface(&window, &mut surface);
                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(width * height) as usize {
                    buffer[index] = pixmap.data()[index * 4 + 2] as u32
                        | (pixmap.data()[index * 4 + 1] as u32) << 8
                        | (pixmap.data()[index * 4] as u32) << 16;
                }
            
                buffer.present().unwrap();
            }
            Event::LoopDestroyed => (),
            _ => (),
        }
    });
}
