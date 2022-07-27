use crate::{joypad::JoypadKey, utils::time::TimeFrame};
use anyhow::Result;
use cartridge::Cartridge;
use glium::glutin::event::KeyboardInput;
use std::{env, time::Duration};

use crate::hardware::Hardware;

mod apu;
mod cartridge;
mod cpu;
mod hardware;
mod joypad;
mod lcd;
mod memory;
mod ppu;
mod renderer;
mod utils;

fn main() -> Result<()> {
    let rom_path: String = env::args().nth(1).expect("No ROM path provided");
    let cartridge = Cartridge::from_path(rom_path)?;
    println!("Running {}", cartridge.header.title);

    let hardware = Hardware::new(cartridge);

    create_window(hardware);

    Ok(())
}

fn create_window(mut hardware: Hardware) {
    use glium::glutin;

    let event_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(320.0, 288.0))
        .with_title("Rustyboy");

    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut time_frame = TimeFrame::new(Duration::from_secs(1) / 60);

    event_loop.run(move |ev, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    handle_key_event(input, &mut hardware);
                }
                _ => (),
            },
            glutin::event::Event::NewEvents(_) => {
                time_frame.update();
                let buffer = hardware.run();

                renderer::render(&display, buffer);
            }
            glutin::event::Event::RedrawEventsCleared => {
                time_frame.wait();
            }
            _ => (),
        }
    });
}

fn handle_key_event(input: KeyboardInput, hardware: &mut Hardware) {
    use glium::glutin::{self, event::VirtualKeyCode};

    match input {
        KeyboardInput {
            virtual_keycode: Some(key),
            state,
            ..
        } => {
            let joypad_key = match key {
                VirtualKeyCode::A => Some(JoypadKey::A),
                VirtualKeyCode::S => Some(JoypadKey::B),
                VirtualKeyCode::Back => Some(JoypadKey::Select),
                VirtualKeyCode::Return => Some(JoypadKey::Start),
                VirtualKeyCode::Up => Some(JoypadKey::Up),
                VirtualKeyCode::Down => Some(JoypadKey::Down),
                VirtualKeyCode::Left => Some(JoypadKey::Left),
                VirtualKeyCode::Right => Some(JoypadKey::Right),
                _ => None,
            };

            if let Some(key) = joypad_key {
                match state {
                    glutin::event::ElementState::Pressed => {
                        hardware.button_pressed(key);
                    }
                    glutin::event::ElementState::Released => {
                        hardware.button_released(key);
                    }
                }
            }

            if let VirtualKeyCode::Escape = key {
                std::process::exit(0);
            }

            if let VirtualKeyCode::T = key {
                hardware.enable_tracing();
            }
        }
        _ => (),
    }
}
