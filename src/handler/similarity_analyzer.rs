use std::path::PathBuf;
use std::collections::HashMap;
use super::calculate_similarity;
use super::handle::HashingType;

pub struct SimilarityAnalyzer {
    // pub dir_path : PathBuf,
    pub hash_type : HashingType,
    pub similarity_map : HashMap<PathBuf, Vec<(PathBuf, u32)>>
}

impl SimilarityAnalyzer {
    // constructor
    pub fn new(dir_path : PathBuf, hash_type : HashingType) -> Self {
        let similarities: HashMap<PathBuf, Vec<(PathBuf, u32)>> = calculate_similarity(&dir_path, hash_type);
        Self {
            // dir_path,
            hash_type,
            similarity_map : similarities
        }
    }

    pub fn get_one_file_similarity(&self, file_path : &PathBuf) -> &Vec<(PathBuf, u32)> {
        &self.similarity_map[file_path]
    }
}