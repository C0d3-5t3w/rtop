# rtop

A powerful system monitoring tool written in Rust. Inspired by top, htop, and btop.

![rtop screenshot](https://via.placeholder.com/800x500?text=rtop+screenshot)

## Features

- Real-time CPU, memory, disk, and network monitoring
- Process management with sorting and filtering
- Customizable UI themes
- Responsive terminal interface
- Low system footprint
- Cross-platform support

## Installation

### From Source

```bash
git clone https://github.com/yourusername/rtop.git
cd rtop
cargo build --release
```

### Command-line Options

```
USAGE:
    rtop [OPTIONS]

OPTIONS:
    -i, --interval <MS>       Update interval in milliseconds [default: 1000]
    -v, --view <VIEW>         Display mode [default: basic]
                              [possible values: basic, detailed, processfocus, systemfocus]
    -t, --theme <THEME>       Color theme [default: default]
                              [possible values: default, dark, light, custom]
    -c, --config <FILE>       Path to configuration file
    -f, --filter <PATTERN>    Process filter string
    -h, --help                Print help information
    -V, --version             Print version information
```

## Configuration

rtop can be customized through a configuration file. See [DOC.md](DOC.md) for detailed documentation.

Example configuration:

```yaml
update_interval: 1000
theme: "dark"
layout:
  show_cpu: true
  show_memory: true
  show_network: true
  show_disk: true
sort_by: "cpu"
```

## Keyboard Controls

| Key | Action |
|-----|--------|
| q | Quit |
| c | Cycle through color themes |
| 1-4 | Switch between layout views |
| h/l | Navigate tabs |
| j/k | Scroll through lists |

For more details, see the [documentation](DOC.md).

## License

This project is licensed under the MIT License - see the LICENSE file for details.
