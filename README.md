# 🖼️ Perceptual Hashing TUI

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](LICENSE)
[![Release](https://img.shields.io/github/v/release/OlaszPL/perceptual-hashing?style=for-the-badge)](https://github.com/OlaszPL/perceptual-hashing/releases)

[![Built With Ratatui](https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff)](https://ratatui.rs/)

> **Blazing fast** 🚀 and **memory-safe** perceptual hashing application built with Rust!

A terminal user interface (TUI) application for computing perceptual hashes of images and finding similar images with convenient preview capabilities. Hash computation is performed using **multi-threading** for optimal performance.

## 👥 Authors

**Project realized as a part of Rust Course at AGH University of Krakow by:**

- **Aleksander Jóźwik** - [[OlaszPL]](https://github.com/OlaszPL)
- **Piotr Kacprzak** - [[pkacprzak5]](https://github.com/pkacprzak5)

<img width="1594" height="1032" alt="image" src="https://github.com/user-attachments/assets/b18f8fc7-2cb3-4974-a3de-1c0ae8fe6506" />

## ✨ Features

- 🖼️ **Perceptual hashing** with two algorithms:
  - **dHash** (Difference Hash)
  - **pHash** (Perceptual Hash)
- 🔍 **Similar image detection** with visual preview
- 📁 **Built-in file explorer** for folder selection
- ⚡ **Multi-threaded processing** for blazing fast performance
- 🎨 **Interactive TUI** built with [Ratatui](https://github.com/ratatui-org/ratatui)
- 🔢 **64-bit hash computation**
- 🖥️ **Cross-platform support** (Linux & Windows)

## 🎯 How it Works

The application provides an intuitive TUI interface with:

1. **Built-in file explorer** - Navigate and select folders containing images
2. **Algorithm selection** - Choose between dHash and pHash algorithms
3. **Results browser** - View similar images with side-by-side preview of source and similar images

## 🎬 Demo

<div align="center">

![Nagranieekranuz2025-10-0313-27-36-ezgif com-video-to-gif-converter](https://github.com/user-attachments/assets/5dc6a231-ab4b-4f51-bcf9-e43d124b7175)


*Complete workflow: folder selection → algorithm choice → processing → results exploration*

</div>

---

## 📋 Prerequisites

- **Rust** (edition 2024 or higher)
- **Cargo** (Rust package manager)

## 📦 Installation

### Download Pre-built Binaries

You can download pre-built binaries for your platform from the [releases page](https://github.com/OlaszPL/perceptual-hashing/releases).

Available architectures:
- **Linux x86_64**
- **Windows x86_64**

> [!IMPORTANT]
> - The **Windows** version does **not** support image display in the terminal.
> - On **Linux**, image display in the terminal is only supported in terminals listed in the  
>   [ratatui-image compatibility matrix](https://github.com/benjajaja/ratatui-image?tab=readme-ov-file#compatibility-matrix).

### Running the Application

#### Linux
```bash
./perceptual-hashing
```

#### Windows
```powershell
.\perceptual-hashing.exe
```

### Build from Source

1. **Clone the repository**
   ```bash
   git clone https://github.com/OlaszPL/perceptual-hashing.git
   cd perceptual-hashing
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Run the application**
   ```bash
   cargo run --release
   ```

## 🔍 Algorithms

The application implements two proven perceptual hashing algorithms:

### **dHash (Difference Hash)**
- **Fast computation** - ideal for real-time processing
- **Good performance** on brightness/contrast variations
- **Lower memory usage**
- **Best for**: Quick similarity detection

### **pHash (Perceptual Hash)** 
- **Higher accuracy** - more robust similarity detection
- **Better resilience** to image transformations
- **DCT-based approach** for perceptual similarity
- **Best for**: Precise duplicate detection

Both algorithms produce **64-bit hashes** and use **Hamming distance** for similarity comparison (lower distance = more similar images).

## 🎮 User Interface

The TUI provides an intuitive workflow:

### 1. **File Selection**
- Browse directories using arrow keys
- Press `c` to select a folder for processing
- Press `q` to exit

<img width="1594" height="1032" alt="image" src="https://github.com/user-attachments/assets/6ae29ef8-8067-46f4-bab6-b2a3f7e3d99e" />

### 2. **Algorithm Selection** 
- Choose between **dHash** (red) and **pHash** (blue)
- Use arrow keys or mouse to toggle selection
- Press `Enter` to confirm

<img width="1594" height="1032" alt="image" src="https://github.com/user-attachments/assets/2f3fdf36-05cc-4f94-8c87-200149d794a9" />

### 3. **Processing**
- Real-time progress with "CALCULATING" indicator
- Processing time displayed in the top-right corner
- Multi-threaded computation for optimal performance

<img width="1594" height="1032" alt="image" src="https://github.com/user-attachments/assets/339d5707-b26a-4405-8452-1b9b3383dfd1" />

### 4. **Results Exploration**
- **Left Panel**: List of all processed images
- **Center Panel**: Similar images with Hamming distance scores (0 = identical)
- **Right Panel**: Side-by-side preview of selected and similar images
- Navigate with arrow keys, press `Esc` to go back

<img width="1594" height="1032" alt="image" src="https://github.com/user-attachments/assets/b18f8fc7-2cb3-4974-a3de-1c0ae8fe6506" />

## 🧪 Testing

The project includes comprehensive unit tests for the hashing algorithms:

```bash
# Run all tests
cargo test
```

Tests cover:
- **dHash algorithm** correctness and consistency
- **pHash algorithm** accuracy and edge cases
- Hash computation validation

## ⚠️ Important Notes

- **Folder permissions**: The application cannot access folders without proper read permissions
- **Minimum images**: Processing requires at least 2 images in the selected directory
- **Hash size**: All computed hashes are 64-bit for optimal performance and accuracy
- **Image formats**: Supports common formats (JPEG, PNG, etc.)
- **Performance**: Processing time scales with image count and selected algorithm

## 🛠️ Technology Stack

- **Language**: Rust 🦀 (memory-safe and blazing fast!)
- **TUI Framework**: [Ratatui](https://github.com/ratatui-org/ratatui)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">
  <strong>Made with ❤️ and 🦀 Rust</strong>
</div>
