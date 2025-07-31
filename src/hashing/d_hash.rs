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
    assert_eq!(img.height(), IMG_HEIGHT as u32, "Image height must be {IMG_HEIGHT}");
    assert_eq!(img.width(), IMG_WIDTH as u32, "Image width must be {IMG_WIDTH}");

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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::hashing::d_hash;


    #[test]
    fn compare_p_hashes_with_library() {
        let test_images_dir = Path::new("test_images");

        for entry in fs::read_dir(test_images_dir).expect("Could not read test_images directory") {
            let entry = entry.expect("Could not read image entry");
            let path = entry.path();

            if path.extension().map(|e| e.to_ascii_lowercase()) == Some("jpg".into())
                || path.extension().map(|e| e.to_ascii_lowercase()) == Some("png".into())
            {
                println!("Testing image: {path:?}");

                let my_hash = d_hash(&path).expect("p_hash failed");

                let file_name = path
                    .file_name()
                    .and_then(|os| os.to_str())
                    .expect("Filename is not valid UTF-8");
                let expected: u64 = match file_name {
                    "test1.png" => 3400005716650440455,
                    "test2.png" => 3400005699470571271,
                    "test3.png" => 12442841909063455382,
                    "test4.png" => 17435395539415830403,
                    other => panic!("Unexpected file name: {other}"),
                };
                assert_eq!(
                    my_hash, expected,
                    "Hashes differ for image {:?}: my_hash={:016x}, lib_hash={:016x}",
                    path.file_name().unwrap(),
                    my_hash,
                    expected
                );
            }
        }
    }
}