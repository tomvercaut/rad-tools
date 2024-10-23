# dcmcp

## Description

A tool to copy DICOM data from one directory to another using the patient ID.

## Build

```shell
cargo build --release
```

## Usage

```shell
A command line interface (CLI) application to copy DICOM files by patient ID.


Usage: dcm_cp.exe [OPTIONS] --patient-id <PATIENT_ID> <SOURCE>... <DST>

Arguments:
  <SOURCE>...
          File(s) or director(y/ies) from where DICOM files are copied (recursively)

  <DST>
          Directory to where DICOM files are copied

Options:
  -p, --patient-id <PATIENT_ID>
          Patient ID (unique patient identifier)

  -v, --verbose
          Enable logging at INFO level

      --debug
          Enable logging at DEBUG level

      --trace
          Enable logging at TRACE level

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```