use std::num::TryFromIntError;

use arboard::Clipboard;
use image::{ImageBuffer, RgbaImage};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No image content available in clipboard")]
    NoImageContent(),
    #[error("Unable to convert clipboard contents to proper image format")]
    RgbaImageError(),
    #[error("Unable to determine size during image conversion")]
    RgbaImageSizeError(#[from] TryFromIntError),
    #[error("Clipboard is occupied by another process")]
    Occupied(),
    #[error("Clipboard is not supported")]
    NotSupported(),
    #[error("Unknown clipboard error")]
    Unknown(),
}

impl From<arboard::Error> for Error {
    fn from(err: arboard::Error) -> Self {
        match err {
            arboard::Error::ContentNotAvailable => Error::NoImageContent(),
            arboard::Error::ClipboardOccupied => Error::Occupied(),
            arboard::Error::ConversionFailure => Error::RgbaImageError(),
            arboard::Error::ClipboardNotSupported => Error::NotSupported(),
            _ => Error::Unknown()
        }
    }
}

pub fn get_image() -> Result<RgbaImage, Error> {
    let image = Clipboard::new()?.get_image()?.to_owned_img();
    let width: u32 = image.width.try_into()?;
    let height: u32 = image.height.try_into()?;

    ImageBuffer::from_raw(
        width,
        height,
        image.bytes.to_vec(),
    ).ok_or(Error::RgbaImageError())
}
