use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use image::ImageError;

use crate::hashing;

#[derive(Debug, Clone, Copy)]
pub enum HashingType {
    DHash,
    PHash,
}

impl HashingType {
    pub fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::DHash),
            1 => Some(Self::PHash),
            _ => None
        }
    }
}

impl std::fmt::Display for HashingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashingType::DHash => write!(f, "dHash"),
            HashingType::PHash => write!(f, "pHash"),
        }
    }
}

fn calculate_hashes(path: &Path, hashing_func: fn(path: &Path) -> Result<u64, ImageError>) -> HashMap<PathBuf, u64> {
    let mut hashes = HashMap::new();
        if let Ok(paths) = fs::read_dir(path) {
            for dir_entry in paths.flatten() {
                let cur_path = dir_entry.path();
                if cur_path.is_file() {
                    if let Ok(cur_hash) = hashing_func(&cur_path) { // fault tolerance
                        hashes.insert(cur_path, cur_hash);
                    }
                }
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