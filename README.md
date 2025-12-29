# MiniPNG
__minipng__ is a tiny, PNG-inspired image file format designed for experimentation and learning for the _CSC-4205 Programmation Orientée Sécurité_ course at Telecom SudParis.

## The Format
It keeps only the core ideas of PNG:
- A fixed **magic header** to identify the file
- A **block-based structure**, where each block has a type and a length
- A mandatory **header block** describing the image
- One or more **data blocks** holding the pixel data
- Optional blocks (comments, palette) for extra information

The format supports multiple **pixel types** (black & white 1 bit, grayscale 8 bits, RGB 24 bits and palette).

## Implementation details
This repository is a CLI built in _Rust_ with _clap_ and _thiserror_ for error management.

## Usage
```bash
> minipng --help

# Usage: minipng <COMMAND>

# Commands:
#   display  
#   encode   
#   help     Print this message or the help of the given subcommand(s)

# Options:
#   -h, --help     Print help
#   -V, --version  Print version
```

```bash
> minipng display --help

# Usage: minipng display [PATHS]...

# Arguments:
#   [PATHS]...  paths of the Mini-PNG images

# Options:
#   -h, --help  Print help
```

```bash
> minipng encode --help

# Usage: minipng encode --output <OUTPUT> <INPUT>

# Arguments:
#   <INPUT>  path of the input text file

# Options:
#   -o, --output <OUTPUT>  path of the output Mini-PNG file
#   -h, --help             Print help
```
