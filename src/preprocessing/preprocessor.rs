use image::{DynamicImage, GrayImage, ImageError, ImageReader, imageops};
use std::path::Path;

pub fn preprocess(path: &Path, height: u32, width: u32) -> Result<GrayImage, ImageError> {
    let img: DynamicImage = ImageReader::open(path)?.decode()?;

    Ok(img
        .grayscale()
        .resize_exact(width, height, imageops::Lanczos3)
        .into_luma8())
}