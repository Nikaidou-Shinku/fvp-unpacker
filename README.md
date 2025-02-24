## FVP Unpacker

A blazing fast tool to unpack FVP archive.

### Features

- List all files in `.bin` archive
- Unpack images from `.bin` archive
- Process images in `.bin` archive, and output the tachie(立ち絵)

#### TODO

- Unpack from/to multiple file formats (from `.bin`, `.hzc` and `.hcb` files, to `.hzc`, `.png`, `.ogg`, etc.)
- Automatic file format detection
- better CLI

### Usage

```console
A blazing fast tool to unpack FVP archive

Usage: fvp-unpacker-cli <COMMAND>

Commands:
  unpack  Unpack all files from the archive without additional processing
  list    List files that can be unpacked
  tachie  Process the original image and output the tachie(立ち絵)
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
