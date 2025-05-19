use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::handler::handle::{select_file, select_folder, HashingType};
use crate::handler::similarity_analyzer::SimilarityAnalyzer;

#[derive(Debug, Copy, Clone)]
enum Mode {
    Single,
    Multi
}

fn select_hashing_type() -> HashingType {
    println!("1. pHash");
    println!("2. dHash");
    print!("Select hashing mode: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => HashingType::PHash,
        "2" => HashingType::DHash,
        _ => {
            println!("Invalid choice, defaulting to pHash.");
            HashingType::PHash
        }
    }
}

fn select_mode() -> Mode {
    println!("1. Single");
    println!("2. Multi");
    print!("Select operation mode: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => Mode::Single,
        "2" => Mode::Multi,
        _ => {
            println!("Invalid choice, defaulting to Multi.");
            Mode::Multi
        }
    }
}

fn print_similarity_multi(analyzer: &SimilarityAnalyzer) {
    println!("\n--- {} Similarity Results ---", analyzer.hash_type);

    let similarity_map = &analyzer.similarity_map;
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
                if original_name == similar_name {
                    continue;
                }
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

fn print_similarity_single(analyzer: &SimilarityAnalyzer, img_path: PathBuf) {
    println!("\n--- {} Similarity Results (Single) ---", analyzer.hash_type);

    let similarities = analyzer.get_one_file_similarity(&img_path);
    let original_name = img_path
        .file_name()
        .map_or_else(|| "N/A".into(), |n| n.to_string_lossy());

    println!("Image: {}", original_name);
    if similarities.is_empty() {
        println!("    No similar images found.");
    } else {
        for (similar_img_path, distance) in similarities {
            let similar_name = similar_img_path
                .file_name()
                .map_or_else(|| "N/A".into(), |n| n.to_string_lossy());
            if original_name == similar_name {
                continue;
            }
            println!(
                "    -> Similar to: {:<30} (Distance: {})",
                similar_name,
                distance
            );
        }
    }
    println!();
}


pub fn run() {
    let hashing_type : HashingType = select_hashing_type();
    let dir_path = match select_folder() {
        Some(path) => path,
        None => {
            println!("Folder selection was canceled. Exiting.");
            return;
        }
    };
    let sim_an : SimilarityAnalyzer = SimilarityAnalyzer::new(dir_path, hashing_type);
    let mode : Mode = select_mode();

    match mode {
        Mode::Single =>
            {
            let file_path = match select_file() {
                Some(path) => path,
                None => {
                    println!("File selection was canceled. Exiting.");
                    return;
                }
            };
            print_similarity_single(&sim_an, file_path);
            },
        Mode::Multi => 
            print_similarity_multi(&sim_an),
    }
}