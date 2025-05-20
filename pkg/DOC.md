# rtop Documentation

rtop is a system monitoring tool written in Rust, inspired by tools like top, htop, and btop.

## Table of Contents

- [rtop Documentation](#rtop-documentation)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
    - [From Source](#from-source)
    - [Using Cargo](#using-cargo)
  - [Usage](#usage)
  - [Command-line Options](#command-line-options)
  - [Configuration](#configuration)
  - [Keyboard Controls](#keyboard-controls)
  - [Customization](#customization)
    - [Themes](#themes)
    - [Layout](#layout)
  - [System Requirements](#system-requirements)
  - [Troubleshooting](#troubleshooting)
    - [High CPU Usage](#high-cpu-usage)
    - [Missing Information](#missing-information)

## Installation

### From Source

```bash
git clone https://github.com/yourusername/rtop.git
cd rtop
cargo build --release
```

The binary will be located at `target/release/rtop`.

### Using Cargo

```bash
cargo install rtop
```

## Usage

Simply run the `rtop` command:

```bash
rtop
```

## Command-line Options

- `-i, --interval <MILLISECONDS>`: Update interval in milliseconds (default: 1000)
- `-v, --view <VIEW>`: Display mode (basic, detailed, processfocus, systemfocus)
- `-t, --theme <THEME>`: Color theme (default, dark, light, custom)
- `-c, --config <FILE>`: Path to configuration file
- `-f, --filter <PATTERN>`: Process filter string

Example:
```bash
rtop --interval 2000 --theme dark --filter "rust"
```

## Configuration

rtop can be configured using a YAML configuration file. By default, rtop looks for a configuration file at:

- `$HOME/.config/rtop/config.yaml`

You can also specify a configuration file with the `--config` option.

See the `config.yaml` file included in the repository for a fully documented example.

## Keyboard Controls

| Key | Action |
|-----|--------|
| q | Quit rtop |
| c | Cycle through color themes |
| 1 | Switch to default layout |
| 2 | Switch to CPU-focused layout |
| 3 | Switch to memory-focused layout |
| 4 | Switch to compact layout |
| h/l | Navigate tabs |
| j/k | Scroll through lists |
| F2 | Edit configuration (when implemented) |
| / | Search (when implemented) |

## Customization

### Themes

rtop comes with several built-in themes:
- default
- dark
- light
- custom (defined in configuration file)

You can customize colors by editing the `custom_theme` section in your configuration file.

### Layout

You can show/hide components by modifying the `layout` section in your configuration file:

```yaml
layout:
  show_cpu: true
  show_memory: true
  show_network: true
  show_disk: true
  show_process_details: true
```

## System Requirements

- Linux, macOS, or Windows
- Rust 1.63.0 or later (for building from source)

## Troubleshooting

### High CPU Usage

If rtop itself is using too much CPU, try increasing the update interval:

```bash
rtop --interval 2000
```

### Missing Information

Some system metrics might require elevated privileges. Try running with sudo:

```bash
sudo rtop
```
