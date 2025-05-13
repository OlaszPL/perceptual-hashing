use crate::preprocessing::preprocessor::preprocess;
use std::path::Path;
use image::GrayImage;

// to produce 64 bit hash
const IMG_HEIGHT: u32 = 9;
const IMG_WIDTH: u32 = 8;

pub fn d_hash(path: &Path) -> u64 {
    let img = preprocess(path, IMG_HEIGHT, IMG_WIDTH).unwrap();
    hash(&img)
}

fn hash(img: &GrayImage) -> u64 {
    assert_eq!(img.height(), IMG_HEIGHT, "Image height must be {}", IMG_HEIGHT);
    assert_eq!(img.width(), IMG_WIDTH, "Image width must be {}", IMG_WIDTH);

    // 1 means that left pixel is brighter than the right pixel, 0 otherwise
    // big-endian save

    let mut hash : u64 = 0;

    for i in 0..IMG_HEIGHT {
        for j in 0..(IMG_WIDTH - 1) {
            hash <<= 1;
            // if left is brighter
            if img.get_pixel(j, i)[0] > img.get_pixel(j + 1, i)[0] {
                hash |= 1;
            }
        }
    }

    hash
}