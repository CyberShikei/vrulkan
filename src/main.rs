#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod venreir;

use crate::venreir::App;

use anyhow::Result;

use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() -> Result<()> {
    pretty_env_logger::init();


    // Window

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(800, 400))
        .build(&event_loop)?;


    // App

    let mut app = unsafe { App::create(&window)? };
    let mut minimized = false;
    event_loop.run(move |event, elwt| {
        match event {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => app.resized = true,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::RedrawRequested if !elwt.exiting() && !minimized => {
                    unsafe { app.render(&window) }.unwrap();
                }
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 {
                        minimized = true;
                    } else {
                        minimized = false;
                        app.resized = true;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    })?;

    Ok(())
}
