use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::hashing;
use rfd::FileDialog;

#[derive(Debug, Clone, Copy)]
pub enum HashingType {
    DHash,
    PHash,
}

impl std::fmt::Display for HashingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashingType::DHash => write!(f, "dHash"),
            HashingType::PHash => write!(f, "pHash"),
        }
    }
}

pub fn select_folder() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Select a folder containing images")
        .pick_folder()
}

pub fn select_file() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Select an image file")
        .pick_file()
}

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

pub fn calculate_similarity(path: &Path, hashing_type: HashingType) -> HashMap<PathBuf, Vec<(PathBuf, u32)>> {
    let hashes_map = match hashing_type {
        HashingType::PHash => calculate_hashes(path, hashing::p_hash),
        HashingType::DHash => calculate_hashes(path, hashing::d_hash),
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
        distances.sort_by_key(|&(_, distance)| distance);
        similarity_results.insert((*path1).clone(), distances);
    }

    similarity_results
}