## FVP Unpacker

A blazing fast tool to unpack FVP archive.

### Features

- Unpack images from `.bin` archive

#### TODO

- Unpack from/to multiple file formats (from `.bin`, `.hzc` and `.hcb` files, to `.hzc`, `.png`, `.ogg`, etc.)
- Automatic file format detection
- better CLI

### Usage

```console
A blazing fast tool to unpack FVP archive

Usage: fvp-unpacker-cli [OPTIONS] --input <INPUT> <COMMAND>

Commands:
  unpack  Unpack all files from the archive without additional processing
  help    Print this message or the help of the given subcommand(s)

Options:
  -i, --input <INPUT>    Input file path
  -o, --output <OUTPUT>  Output directory path [default: ./output]
  -h, --help             Print help
  -V, --version          Print version
```
