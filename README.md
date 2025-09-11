# cnav

[![Crates.io](https://img.shields.io/crates/v/cnav.svg)](https://crates.io/crates/cnav)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

cnav (Code NAVigator) is a fast, terminal-based code navigation tool powered by ripgrep.
It helps you quickly jump to functions, variables, or any symbol in your project, directly inside your favorite terminal editor.

## Features

- **Lightning Fast:** Powered by ripgrep for instant symbol searches.
- **Interactive Selection:** Browse through matches with an intuitive TUI.
- **Editor Integration:** Opens files directly in your favorite terminal editor.
- **Tmux Support:** Seamlessly integrates with tmux workflows.
- **Syntax Highlighting:** Highlights matched symbols in search results.
- **Configurable:** Supports multiple editors and customization options

## Requirements

- Rust (1.72 or higher)
- ripgrep
- A terminal editor (vim, nvim, nano, emacs, micro, helix, kakoune, joe)
- Optional: tmux (for tmux split support)

## Installation

From [Crates.io](https://crates.io/)

```bash
cargo install cnav
```
From source

```
git clone https://github.com/agace/cnav
cd cnav
cargo install --path .
```

## Usage

```bash
cnav [OPTION] <path> <symbol>
```

## Options

- `-n, --no-interactive`: Open the first match directly without interactive selection
- `-t, --tmux`: Open the file in a new tmux split (when inside tmux)
- `-e, --editor <EDITOR>`: Specify the terminal editor to use
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## Examples

1. Interactive search for a function:

```bash
cnav /path/to/project main
```

2. Open first result directly:

```bash
cnav -n src/utils calculate
```

3. Use with tmux and specific editor:

```bash
cnav -t -e nvim . my_function
```

## Keybindings (Interactive Mode)

- j / ↓ - move down
- k / ↑ - move up
- h / ← - previous page
- l / → - next page
- Enter - open selected match
- q - quit without opening

## Editor Support

| Editor           | Line Jump Syntax        |
|------------------|-------------------------|
| vim, nvim        | `vim +42 file.txt`      |
| nano             | `nano +42 file.txt`     |
| emacs            | `emacs +42 file.txt`    |
| micro            | `micro file.txt:+42`    |
| helix (hx)       | `hx file.txt:42`        |
| kakoune (kak)    | `kak +42 file.txt`      |
| joe              | `joe +42 file.txt`      |

> Other editors still open, but may not support direct line jumps.

## Editor Configuration

Set your preferred editor using the EDITOR environment variable:

```bash
export EDITOR="nvim"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
