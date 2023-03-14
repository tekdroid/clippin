use std::num::TryFromIntError;

use image::RgbaImage;
use softbuffer::{Context, Surface};
use winit::{
    error::OsError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::{WindowButtons, WindowLevel},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to open window")]
    OpenError(#[from] OsError),
    #[error("Unable to open window")]
    SoftBufferError(),
    #[error("Unable to determine size during image conversion")]
    RgbaImageSizeError(#[from] TryFromIntError),
}

impl From<softbuffer::SoftBufferError> for Error {
    fn from(_: softbuffer::SoftBufferError) -> Self {
        Error::SoftBufferError()
    }
}

fn open_image_window(buffer: Vec<u32>, width: u16, height: u16) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
        .with_resizable(false)
        .with_decorations(true)
        .with_window_level(WindowLevel::AlwaysOnTop)
        .with_enabled_buttons(WindowButtons::CLOSE)
        .build(&event_loop)?;

    let context = unsafe { Context::new(&window) }?;
    let mut surface = unsafe { Surface::new(&context, &window) }?;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                surface.set_buffer(&buffer, width, height);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

pub fn open_rgba(image: RgbaImage) -> Result<(), Error> {
    let buffer: Result<Vec<u32>, TryFromIntError> = image
        .pixels()
        .map(|rgb| -> Result<u32, TryFromIntError> {
            let red: u32 = rgb.0[0].try_into()?;
            let green: u32 = rgb.0[1].try_into()?;
            let blue: u32 = rgb.0[2].try_into()?;

            Ok(blue | (green << 8) | (red << 16))
        })
        .collect();
    open_image_window(buffer?, image.width() as u16, image.height() as u16)
}
