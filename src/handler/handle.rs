use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::hashing;

fn calculate_hashes(path: &Path, hashing_func: fn(path: &Path) -> u64) -> HashMap<PathBuf, u64> {
    let mut hashes = HashMap::new();
    match fs::read_dir(path) {
        Ok(paths) => {
            for entry in paths {
                match entry {
                    Ok(dir_entry) => {
                        let cur_path = dir_entry.path();
                        if cur_path.is_file() {
                            let cur_hash = hashing_func(&cur_path);
                            hashes.insert(cur_path, cur_hash);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading directory entry in {}: {}", path.display(), e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory {}: {}", path.display(), e);
        }
    }
    hashes
}

pub fn calculate_similarity(path: &Path, hashing_type: i8) -> HashMap<PathBuf, Vec<(PathBuf, u32)>> {
    // hashing_type: 1 -> p_hash, 0 -> d_hash
    if hashing_type != 0 && hashing_type != 1 {
        eprintln!("Wrong hashing_type, expected 0(d_hash) or 1(p_hash), got {}", hashing_type);
        return HashMap::new();
    }

    let hashes_map = if hashing_type == 1 {
        calculate_hashes(path, hashing::p_hash)
    } else {
        calculate_hashes(path, hashing::d_hash)
    };

    if hashes_map.len() < 2 {
        if hashes_map.is_empty() {
            println!("No image hashes were calculated from the directory: {}", path.display());
        } else {
            println!("Only one image hash was calculated. Need at least two images to find similarities.");
        }
        return HashMap::new();
    }

    let image_entries: Vec<(&PathBuf, &u64)> = hashes_map.iter().collect();
    let mut similarity_results: HashMap<PathBuf, Vec<(PathBuf, u32)>> = HashMap::new();

    for (path1, hash1) in image_entries.iter() {
        let mut distances = Vec::new();
        for (path2, hash2) in image_entries.iter() {
            let distance = (**hash1 ^ **hash2).count_ones();
            distances.push(((*path2).clone(), distance));
        }
        similarity_results.insert((*path1).clone(), distances);
    }

    similarity_results
}