# dcm_json

A command-line tool to convert a DICOM file into JSON format.

## Description

`dcm2json` is a utility that converts a DICOM file into JSON format.
This tool makes it easier to inspect, process, and integrate DICOM metadata with modern data processing pipelines.

## Build

```shell
cargo build --release
```
## Usage

```shell
dcm2json --help
A command line interface (CLI) application for converting a DICOM file to JSON format

Usage: dcm2json [FILE]

Arguments:
  [FILE]  Filename to a DICOM file, if not specified, the filename will read from standard input

Options:
  -h, --help     Print help
  -V, --version  Print version
```