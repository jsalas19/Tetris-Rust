# Tetris Rust

A lightweight desktop Tetris game built primarily in Rust, with a native GUI
powered by [Macroquad](https://github.com/not-fl3/macroquad). Play with
keyboard controls, clear lines, and aim for a high score as the game speeds up
with each level.

## Features

- Standard 10-by-20 playfield and all seven tetrominoes
- Shuffled seven-piece bag, next-piece preview, and landing ghost
- Keyboard movement, rotation, soft drop, and hard drop
- Line clearing, score, and a level that increases every 10 lines
- Pause, resume, and restart

## Play the Windows executable

A prebuilt 64-bit Windows executable is available at:

```text
target\release\tetris-rust.exe
```

Double-click `tetris-rust.exe` to play. This executable is built for Windows;
other operating systems need their own build.

## Build and run from source

1. Install the stable Rust toolchain using [rustup](https://rustup.rs/).
2. Clone or download this project and open a terminal in the project folder
   (the folder containing `Cargo.toml`).
3. Run the game:

   ```powershell
   cargo run
   ```

   The first build downloads the Rust dependencies listed in `Cargo.toml`.

To build an optimized executable without starting the game:

```powershell
cargo build --release
```

The executable is written to `target\release\tetris-rust.exe` on Windows.
To build and launch the optimized version instead, run:

```powershell
cargo run --release
```

### Windows build requirements

On Windows, Rust uses the MSVC toolchain by default. Install Visual Studio or
Visual Studio Build Tools with the **Desktop development with C++** workload
and a Windows SDK. When building from a regular terminal, Cargo must be able to
find the MSVC linker and SDK libraries. If it cannot find `link.exe` or
`kernel32.lib`, open a **Developer PowerShell for Visual Studio** (or a
**Developer Command Prompt for Visual Studio**) and rerun the Cargo command.

If `cargo` is not recognized after installing Rust, reopen the terminal (or
VS Code) so it refreshes your PATH. Rustup normally adds
`%USERPROFILE%\.cargo\bin` to the user PATH.

## Controls

| Action | Keys |
| --- | --- |
| Move left/right | Left/Right arrows or A/D |
| Rotate clockwise | Up arrow or X |
| Rotate counterclockwise | Z |
| Soft drop | Down arrow or S |
| Hard drop | Space |
| Pause/resume | P |
| Restart | R |

## Project files

- `Cargo.toml` — package metadata and dependencies
- `Cargo.lock` — exact dependency versions used for reproducible builds
- `src/main.rs` — game logic, rendering, and application entry point
- `target/` — generated build output, including the Windows executable
