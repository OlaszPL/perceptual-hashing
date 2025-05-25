use crate::preprocessing::preprocessor::preprocess;
use std::path::Path;
use image::{GrayImage, ImageError};

// to produce 64 bit hash
const IMG_HEIGHT: usize = 8;
const IMG_WIDTH: usize = 9;

pub fn d_hash(path: &Path) -> Result<u64, ImageError> {
    let img = preprocess(path, IMG_HEIGHT as u32, IMG_WIDTH as u32)?;
    Ok(hash(&img))
}

fn hash(img: &GrayImage) -> u64 {
    assert_eq!(img.height(), IMG_HEIGHT as u32, "Image height must be {}", IMG_HEIGHT);
    assert_eq!(img.width(), IMG_WIDTH as u32, "Image width must be {}", IMG_WIDTH);

    // 1 means that left pixel is brighter than the right pixel, 0 otherwise
    // big-endian save

    let pixels = img.as_raw();
    let mut row_start: usize;
    let mut hash : u64 = 0;

    for i in 0..IMG_HEIGHT {
        row_start = i * IMG_WIDTH;
        for j in 0..(IMG_WIDTH - 1) {
            hash <<= 1;
            // if left is brighter
            if pixels[row_start + j] > pixels[row_start + j + 1] {
                hash |= 1;
            }
        }
    }

    hash
}