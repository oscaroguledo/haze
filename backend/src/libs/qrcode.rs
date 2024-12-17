use qrcode::QrCode;
use image::{Luma, ImageBuffer};

use std::fmt;

#[derive(Debug)]
pub enum QrCodeError {
    InvalidData(String),
}

impl fmt::Display for QrCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QR code error: {:?}", self)
    }
}

impl std::error::Error for QrCodeError {}

impl From<qrcode::types::QrError> for QrCodeError {
    fn from(err: qrcode::types::QrError) -> Self {
        QrCodeError::InvalidData(format!("QR Code error: {}", err))
    }
}

pub fn generate(data: &str, output_path: &str, size: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
    // Set the size of the QR code (default to 200 if not provided)
    let size = size.unwrap_or(200);

    // Create the QR code, mapping the error type to our custom error
    let code = QrCode::new(data).map_err(QrCodeError::from)?;

    // Render the QR code with grayscale colors (black is 0, white is 255)
    let image: ImageBuffer<Luma<u8>, Vec<u8>> = code.render::<Luma<u8>>()
    .min_dimensions(size as u32, size as u32)
    .build();


    // Save the image as a PNG file
    image.save(output_path)?;

    println!("QR code saved to {}", output_path);
    Ok(())
}
