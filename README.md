<!-- LOGO -->
<h1>
<p align="center">
  <img src="extra/logo/westerm-logo.png" alt="Logo" width="128">
  <br>Westerm
</h1>
  <p align="center">
    Fast, cross-platform, GPU-accelerated terminal emulator.
    <br />
    <a href="#about">About</a>
    ·
    <a href="https://github.com/deltartificial/westerm/releases">Download</a>
    ·
    <a href="#installation">Installation</a>
    ·
    <a href="#features">Features</a>
    ·
    <a href="#configuration">Configuration</a>
  </p>
</p>

## About

Westerm is a modern terminal emulator that differentiates itself by being
fast, cross-platform, and GPU-accelerated. Built on top of OpenGL, Westerm
provides exceptional performance while maintaining compatibility with all
existing shells and terminal software.

Westerm is a fork of Alacritty, focusing on delivering a seamless terminal
experience across macOS, Linux, Windows, and BSD systems. It comes with
sensible defaults but allows for extensive configuration to match your
workflow.

While aiming for this ambitious goal, Westerm strives to be one of the best
fully standards-compliant terminal emulators, remaining compatible with all
existing shells and software while supporting all of the latest terminal
innovations in the ecosystem. You can use Westerm as a drop-in replacement
for your existing terminal emulator.

**Key Features:**
- **GPU Acceleration**: Leverages OpenGL for smooth, efficient rendering
- **Cross-Platform**: Native support for macOS, Linux, Windows, and BSD
- **Highly Configurable**: Extensive configuration options via TOML
- **Standards Compliant**: Full VT100/ANSI terminal emulation
- **Lightweight**: Minimal dependencies and resource usage
- **Modern**: Support for modern terminal features and protocols

The software is considered to be at a **beta** level of readiness; there are
a few missing features and bugs to be fixed, but it is already used by many as
a daily driver.

## Installation

### macOS

Download the latest `.app` from the [GitHub releases page](https://github.com/deltartificial/westerm/releases),
or build from source:

```bash
# Clone the repository
git clone https://github.com/deltartificial/westerm.git
cd westerm

# Build and install
make app
cp -r target/release/osx/Westerm.app /Applications/
```

### Linux

```bash
# Clone the repository
git clone https://github.com/deltartificial/westerm.git
cd westerm

# Build with cargo
cargo build --release

# Install (optional)
sudo cp target/release/westerm /usr/local/bin/
```

For detailed installation instructions for all platforms, see [INSTALL.md](INSTALL.md).

## Features

Westerm provides a rich set of features while maintaining high performance:

- **GPU Rendering**: Hardware-accelerated text rendering using OpenGL
- **True Color Support**: 24-bit color support
- **Vi Mode**: Built-in vi-mode for keyboard-based text selection
- **Search**: Regex-based search with highlighting
- **Hints**: URL and path detection with keyboard hints
- **Mouse Support**: Click to select, middle-click to paste
- **Scrollback**: Configurable scrollback buffer
- **Multiple Windows**: Native multi-window support
- **Tabs**: Native tab support on macOS
- **Ligatures**: Font ligature support
- **Unicode**: Full Unicode support including emoji
- **Hyperlinks**: OSC 8 hyperlink support
- **Sixel Graphics**: Experimental sixel graphics support

For a complete overview of features, see [docs/features.md](./docs/features.md).

## Configuration

Westerm is configured through a TOML configuration file. The configuration file
location depends on your platform:

- **macOS**: `~/.config/westerm/westerm.toml`
- **Linux**: `~/.config/westerm/westerm.toml`
- **Windows**: `%APPDATA%\westerm\westerm.toml`

Example configuration:

```toml
[window]
opacity = 0.95
padding.x = 10
padding.y = 10

[font]
size = 14.0
normal.family = "JetBrains Mono"

[colors.primary]
background = "#1e1e2e"
foreground = "#cdd6f4"

[cursor]
style = "Beam"
```

For complete configuration documentation, see the [configuration guide](https://github.com/alacritty/alacritty/blob/master/extra/man/alacritty.5.scd).

## Building from Source

### Prerequisites

- **Rust**: Latest stable version (install via [rustup](https://rustup.rs/))
- **CMake**: Required for some dependencies
- **Python 3**: Required for some build scripts

### Build

```bash
# Clone the repository
git clone https://github.com/deltartificial/westerm.git
cd westerm

# Build in release mode
cargo build --release

# The binary will be at target/release/westerm
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open
issues for bugs and feature requests.

## License

Westerm is released under the [Apache License, Version 2.0](LICENSE-APACHE).

## Credits

Westerm is a fork of [Alacritty](https://github.com/alacritty/alacritty),
originally created by Joe Wilm and maintained by Christian Duerr.

Special thanks to all the contributors of the Alacritty project whose
work made Westerm possible.

---

<p align="center">Made with ❤️ by the Westerm community</p>
