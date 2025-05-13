use std::collections::HashMap;
use std::path::PathBuf;

mod hashing;
mod preprocessing;
mod handler;

fn print_similarity_map(description: &str, similarity_map: &HashMap<PathBuf, Vec<(PathBuf, u32)>>) {
    println!("\n--- {} ---", description);

    if similarity_map.is_empty() {
        println!(
            "No similarities were found or calculated (e.g., not enough images or directory empty/invalid)."
        );
    } else {
        for (img_path, similarities) in similarity_map {
            let original_name = img_path
                .file_name()
                .map_or_else(|| "N/A".into(), |n| n.to_string_lossy());

            println!("Image: {}", original_name);
            for (similar_img_path, distance) in similarities {
                let similar_name = similar_img_path
                    .file_name()
                    .map_or_else(|| "N/A".into(), |n| n.to_string_lossy());
                println!(
                    "    -> Similar to: {:<30} (Distance: {})",
                    similar_name,
                    distance
                );
            }
            println!();
        }
    }
}

fn main() {
    let example_dir_path = PathBuf::from("ex1");

    let p_hash_similarity = handler::calculate_similarity(&example_dir_path, 1); // 1 for p_hash
    print_similarity_map("pHash Similarity Results", &p_hash_similarity);

    let d_hash_similarity = handler::calculate_similarity(&example_dir_path, 0); // 0 for d_hash
    print_similarity_map("dHash Similarity Results", &d_hash_similarity);
    
    
}