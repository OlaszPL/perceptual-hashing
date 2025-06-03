use std::path::Path;
use image::{GrayImage, ImageError};
use rustdct::{DctPlanner, Dct2};
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::preprocessing::preprocessor::preprocess;

const HASH_SIZE: usize = 8;
const IMG_SIZE: u32 = 32;

static DCT2_IMG: Lazy<Arc<dyn Dct2<f64>>> = Lazy::new(|| {
    let mut planner = DctPlanner::<f64>::new();
    planner.plan_dct2(IMG_SIZE as usize)
});

pub fn p_hash(path: &Path) -> Result<u64, ImageError> {
    let img: GrayImage = preprocess(path, IMG_SIZE, IMG_SIZE)?;
    Ok(hash(&img))
}

fn hash(img: &GrayImage) -> u64 {
    assert_eq!(img.width(), IMG_SIZE, "Image width must be {}", IMG_SIZE);
    assert_eq!(img.height(), IMG_SIZE, "Image height must be {}", IMG_SIZE);

    let pixels_f64: Vec<f64> = img.pixels().map(|p| p[0] as f64).collect();

    let dct_coeffs = calculate_2d_dct(&pixels_f64, IMG_SIZE as usize);

    let mut low_freq_coeffs = Vec::with_capacity(HASH_SIZE * HASH_SIZE);
    for r in 0..HASH_SIZE {
        for c in 0..HASH_SIZE {
            let index = r * (IMG_SIZE as usize) + c;
            low_freq_coeffs.push(dct_coeffs[index]);
        }
    }

    let mut sorted_coeffs = low_freq_coeffs.clone();
    sorted_coeffs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mid = sorted_coeffs.len() / 2;
    let median = if sorted_coeffs.len() % 2 == 0 {
        (sorted_coeffs[mid - 1] + sorted_coeffs[mid]) / 2.0
    } else {
        sorted_coeffs[mid]
    };

    let mut hash: u64 = 0;
    for (i, &coeff) in low_freq_coeffs.iter().enumerate() {
        if coeff >= median {
            hash |= 1 << i;
        }
    }

    hash
}

fn calculate_2d_dct(pixels: &[f64], size: usize) -> Vec<f64> {
    let dct = &*DCT2_IMG;

    let mut buffer = pixels.to_vec();

    for row_chunk in buffer.chunks_mut(size) {
        dct.process_dct2(row_chunk);
    }

    // transposition of matrix
    let mut transposed = vec![0.0; size * size];
    for r in 0..size {
        for c in 0..size {
            transposed[c * size + r] = buffer[r * size + c];
        }
    }

    for col_chunk in transposed.chunks_mut(size) {
        dct.process_dct2(col_chunk);
    }

    transposed
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use image::io::Reader as ImageReader;
    use crate::hashing::p_hash;
    use img_hash::{HasherConfig, ImageHash};
    use img_hash::image::{DynamicImage, GrayImage, GenericImageView, ImageError};


    #[test]
    fn compare_p_hashes_with_library() {
        let test_images_dir = Path::new("test_images");
    
        let hasher = HasherConfig::new()
            .hash_size(8, 8) // 8x8 = 64-bit jak u Ciebie
            .preproc_dct()
            .to_hasher();
    
        for entry in fs::read_dir(test_images_dir).expect("Could not read test_images directory") {
            let entry = entry.expect("Could not read image entry");
            let path = entry.path();
    
            if path.extension().map(|e| e.to_ascii_lowercase()) == Some("jpg".into())
                || path.extension().map(|e| e.to_ascii_lowercase()) == Some("png".into())
            {
                println!("Testing image: {:?}", path);

                let img_data = ImageReader::open(&path)
                    .unwrap()
                    .decode()
                    .unwrap()
                    .to_rgb8();
                
                let img = img_hash::image::DynamicImage::ImageRgb8(
                    img_hash::image::ImageBuffer::from_raw(
                        img_data.width(),
                        img_data.height(),
                        img_data.into_raw()
                    ).unwrap()
                );
                
                let lib_hash = hasher.hash_image(&img);


                let my_hash = p_hash(&path).expect("p_hash failed");
    
                let lib_hash_u64 = {
                    let bytes = lib_hash.as_bytes();
                    assert_eq!(bytes.len(), 8, "Expected 64-bit hash");
                    u64::from_be_bytes(bytes.try_into().unwrap())
                };
    
                assert_eq!(
                    my_hash, lib_hash_u64,
                    "Hashes differ for image {:?}: my_hash={:016x}, lib_hash={:016x}",
                    path.file_name().unwrap(),
                    my_hash,
                    lib_hash_u64
                );
            }
        }
    }
}
