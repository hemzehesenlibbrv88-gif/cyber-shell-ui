# 🚀 Cyber Shell UI

A modern Linux-like terminal emulator with animated lock screen!

## Features

🔒 **Lock Screen with Clock**
- Beautiful centered clock display
- Live date and time
- Press any key to unlock

✨ **Unlock Animation**
- Smooth upward scroll animation (0.5s)
- Terminal fades in during unlock
- Sleek visual transition

🎮 **Terminal Interface**
- Green-on-black hacker aesthetic
- Command history (Up/Down arrows)
- Full cursor navigation
- Real-time command execution

## Installation

### Build

```bash
git clone https://github.com/hemzehesenlibbrv88-gif/cyber-shell-ui.git
cd cyber-shell-ui
cargo build --release
```

### Run

```bash
# Development
cargo run

# Release (standalone EXE)
./target/release/cyber-shell-ui
```

## Commands

- `help` - Show available commands
- `clear` - Clear terminal
- `ls` - List files
- `pwd` - Print working directory
- `echo <text>` - Echo text
- `date` - Show date/time
- `exit` - Exit (Ctrl+C)

## Controls

| Key | Action |
|-----|--------|
| Any Key (Lock Screen) | Unlock |
| `Enter` | Execute command |
| `↑` / `↓` | History navigation |
| `←` / `→` | Cursor movement |
| `Ctrl+C` | Exit |

---

Made with ❤️ in Rust 🦀
