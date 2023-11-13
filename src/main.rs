mod data;
mod services;
mod utils;

extern crate rodio;
extern crate winit;

use data::collection::BasicPiano;
use services::{generate_flacs, sound_generation::play_sound};
use softbuffer::{Context, Surface};
use std::{path::Path, sync::Arc};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};
use utils::util::load_icon;
use winit::{
    event::{ElementState, Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::num::NonZeroU32;
use async_std::task;
use rodio::OutputStream;

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/piano.jpg");
    let icon = load_icon(Path::new(path));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_theme(Some(winit::window::Theme::Dark))
        .with_title("ДЕЛАЙ ДЕЛАЙ 🥵")
        .with_window_icon(Some(icon))
        .build(&event_loop)
        .unwrap();
    let context = unsafe { Context::new(&window) }.unwrap();
    let mut surface = unsafe { Surface::new(&context, &window) }.unwrap();

    let (stream, handle) = OutputStream::try_default().unwrap();
    let styles = vec![BasicPiano::init_basic(), BasicPiano::init_lancer()];

    generate_flacs(styles);

    // Create an Arc (atomic reference counter) for thread-safe access to handle
    let handle_arc = Arc::new(handle);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

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
                                        play_sound(handle_clone, keycode).await;
                                    });
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
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut pixmap = Pixmap::new(width, height).unwrap();
                pixmap.fill(Color::WHITE);
                let path = PathBuilder::from_circle(
                    (width / 2) as f32,
                    (height / 2) as f32,
                    (width.min(height) / 2) as f32,
                )
                .unwrap();
                let mut paint = Paint::default();
                paint.set_color_rgba8(0, 128, 128, 255);
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::EvenOdd,
                    Transform::identity(),
                    None,
                );
                paint.set_color_rgba8(255, 0, 0, 255);
                let mut stroke = Stroke::default();
                stroke.width = 10.0;
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

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
