# DoseCHECK PDF mv

## Description

This is a tool for moving DoseCHECK PDF files from one directory to another. It can also watch for changes in the input directory and move new files as they are created.

## Build

Build only this package:

```shell
cargo build --release -p "rad-tools-dosecheck-pdf-mv"
```

## Usage

```shell
Move dosecheck PDFs from one directory to another

Usage: dosecheck_pdf_mv [OPTIONS] --input <INPUT_DIR> --output <OUTPUT_DIR>

Options:
  -i, --input <INPUT_DIR>     Input directory
  -o, --output <OUTPUT_DIR>   Output directory
  -w, --watch <MILLISECONDS>  Watch for changes in the input directory (watch interval in milliseconds).
  -h, --help                  Print help
  -V, --version               Print version
```