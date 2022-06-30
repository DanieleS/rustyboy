use anyhow::Result;
use cartridge::Cartridge;
use std::env;

use crate::{hardware::Hardware, utils::performance::mesure_performance};

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
    println!("Running {}", cartridge.title);

    let hardware = Hardware::new(cartridge);

    create_window(hardware);

    Ok(())
}

fn create_window(mut hardware: Hardware) {
    use glium::glutin;

    let event_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(256.0, 256.0))
        .with_title("Rustyboy");

    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(_) => {
                let buffer = hardware.run();

                renderer::render(&display, buffer);
                return;
            }
            _ => (),
        }
    });
}
