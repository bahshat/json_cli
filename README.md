# Manim JSON Generator (mgen) 🎬

A blazing fast, interactive CLI tool built in Rust to generate scene configurations for Manim animations.

## ✨ Features
- **Interactive Prompts**: No more manual JSON syntax errors.
- **Scene Layouts**: Supports `grid`, `quote`, `hero`, `comparison`, and `bullets`.
- **Dynamic Bullets**: Add as many points as you need for bullet layouts.
- **Instant Output**: Generates a `scenes.json` file ready for Manim.

## 🚀 Installation (Arch Linux)

### Prerequisites
Ensure you have the Rust toolchain installed:
```bash
sudo pacman -S rustup
rustup default stable
```

### Build from Source
1. Clone the repository:
   ```bash
   git clone https://github.com
   cd json_cli
   ```
2. Build the production binary:
   ```bash
   cargo build --release
   ```
3. Install to your system path:
   ```bash
   sudo cp target/release/json_cli /usr/local/bin/mgen
   ```

## 🛠 Usage
Simply run the command from any terminal:
```bash
mgen
```
Follow the interactive prompts to build your scene list. Once finished, a `scenes.json` file will be created in your current directory.

## 📄 Sample Output
```json
[
  {
    "layout": "hero",
    "text": "Big Universe",
    "duration": 5
  },
  {
    "layout": "bullets",
    "text": "Atheist?",
    "duration": 10,
    "points": ["No Creator", "No Purpose"]
  }
]
```

## 📜 License
MIT
