# 🏰 Chess CLI

A blazingly fast command-line chess game written in Rust that plays against Stockfish engine.

## 🎯 Features

- **Terminal-based chess interface** with Unicode pieces
- **Stockfish integration** for challenging AI gameplay
- **Standard algebraic notation** for move input
- **Cross-platform support** (Linux, Windows, macOS)
- **Colorful board display** with coordinate system
- **Real-time game state tracking**

## 📋 Prerequisites

### Required: Stockfish Engine

You **MUST** install Stockfish before running this game:

#### Linux (Arch/Manjaro)
```bash
sudo pacman -S stockfish
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt install stockfish
```

#### Windows
1. Download from [Stockfish official site](https://stockfishchess.org/download/)
2. Extract and add to PATH
3. Or place `stockfish.exe` in same folder as chess-cli

#### macOS
```bash
brew install stockfish
```

### Build Requirements

- Rust 1.70+ (`rustup install stable`)
- Git

## 🚀 Installation

### From Source
```bash
git clone https://github.com/kernbit/chess-cli
cd chess-cli
cargo build --release
./target/release/chess-cli --help
```

### From AUR (Arch Linux)
```bash
git clone https://aur.archlinux.org/chess-cli.git
cd chess-cli
makepkg -si
```

## 🎮 Usage

### Basic Commands

```bash
# Play as white pieces
chess-cli --player-white

# Play as black pieces (default)
chess-cli

# Custom Stockfish path
chess-cli --engine-path /path/to/stockfish

# Set thinking time (milliseconds)
chess-cli --time-limit 2000
```

### Game Controls

| Input | Action |
|-------|--------|
| `e4`, `d5` | Pawn moves |
| `Nf3`, `Bb5` | Piece moves |
| `O-O` | Kingside castling |
| `O-O-O` | Queenside castling |
| `e8=Q` | Pawn promotion |
| `help` or `h` | Show help |
| `quit` or `q` | Exit game |

### Example Gameplay

```
🏰 Chess CLI - Playing against Stockfish
Player: White | Engine: Black
Enter moves in algebraic notation (e.g., e4, Nf3, O-O)
Type 'quit' to exit, 'help' for commands

┌───┬───┬───┬───┬───┬───┬───┬───┐
│ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │ ♝ │ ♞ │ ♜ │ 8
├───┼───┼───┼───┼───┼───┼───┼───┤
│ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ 7
├───┼───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │   │ 6
├───┼───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │   │ 5
├───┼───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │   │ 4
├───┼───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │   │ 3
├───┼───┼───┼───┼───┼───┼───┼───┤
│ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ 2
├───┼───┼───┼───┼───┼───┼───┼───┤
│ ♖ │ ♘ │ ♗ │ ♕ │ ♔ │ ♗ │ ♘ │ ♖ │ 1
└───┴───┴───┴───┴───┴───┴───┴───┘
  a   b   c   d   e   f   g   h

Turn: White | Move: 1

Enter your move: e4
✓ Move played: e4
🤖 Stockfish is thinking...
🤖 Stockfish plays: e5
```

## ⚙️ Configuration

### Engine Settings

The game automatically detects Stockfish in these locations:

**Linux/macOS:**
- `/usr/bin/stockfish`
- `/usr/local/bin/stockfish`
- `stockfish` (in PATH)

**Windows:**
- `stockfish.exe` (in PATH)
- `stockfish.exe` (same directory)
- `C:\Program Files\Stockfish\stockfish.exe`

### Custom Engine Path

```bash
# Linux/macOS
chess-cli --engine-path /opt/stockfish/bin/stockfish

# Windows
chess-cli --engine-path "C:\Chess\stockfish.exe"
```

## 🔧 Building for Distribution

### Release Build
```bash
cargo build --release --strip
```

### Cross-compilation

#### For Windows (from Linux)
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

#### For macOS (from Linux)
```bash
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

## 📦 AUR Package

### PKGBUILD for Arch Linux

```bash
# Build package
makepkg -f

# Install locally
sudo pacman -U chess-cli-*.pkg.tar.xz

# Or submit to AUR
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Initial commit"
git push
```

## 🐛 Troubleshooting

### Stockfish Not Found
```
❌ Error loading Stockfish: No such file or directory
```

**Solutions:**
1. Install Stockfish: `sudo pacman -S stockfish`
2. Add Stockfish to PATH
3. Use `--engine-path` flag with full path

### Unicode Issues (Windows)
If chess pieces don't display correctly:

1. Use Windows Terminal or modern terminal
2. Set font to "Cascadia Code" or "JetBrains Mono"
3. Enable UTF-8: `chcp 65001`

### Performance Issues
```bash
# Reduce thinking time
chess-cli --time-limit 500

# Use faster engine settings
chess-cli --engine-path stockfish-lite
```

## 🏗️ Development

### Project Structure
```
src/
├── main.rs          # CLI interface and game loop
├── chess/           # Chess game logic
│   ├── mod.rs       # Module exports
│   ├── board.rs     # Board representation
│   ├── pieces.rs    # Piece definitions
│   ├── moves.rs     # Move generation
│   └── position.rs  # Position handling
├── engine.rs        # Stockfish UCI interface
└── ui.rs           # Terminal UI rendering
```

### Running Tests
```bash
cargo test
cargo test --release
```

### Code Formatting
```bash
cargo fmt
cargo clippy
```

## 📝 License

GNU  License - see [LICENSE](LICENSE) file.

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature-name`
3. Commit changes: `git commit -am 'Add feature'`
4. Push branch: `git push origin feature-name`
5. Submit pull request

## 🎯 Roadmap

- [ ] Complete chess rules implementation
- [ ] Engine strength adjustment
- [ ] Game save/load functionality
- [ ] PGN export
- [ ] Opening book integration
- [ ] Multiple engine support
- [ ] Tournament mode

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/kernbit/chess-cli/issues)
- **Discussions:** [GitHub Discussions](https://github.com/kernbit/chess-cli/discussions)
- **AUR Comments:** [AUR Package Page](https://aur.archlinux.org/packages/chess-cli)

---

**⚠️ Remember: Install Stockfish first or the game won't work!**

Made with ❤️ in Rust by kernbit
