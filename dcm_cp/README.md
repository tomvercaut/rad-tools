# dcm_cp

## Description

A tool to copy DICOM data from one directory to another using the patient ID.

## Build

```shell
cargo build --release -p rad-tools-dcm-cp
```

## Install

```shell
cargo install --path dcm_cp
````

## Usage

```shell
dcm_cp --help
A command line interface (CLI) application to copy DICOM files by patient ID.


Usage: dcm_cp --patient-id <PATIENT_ID> <SOURCE>... <DST>

Arguments:
  <SOURCE>...
          File(s) or director(y/ies) from where DICOM files are copied (recursively)

  <DST>
          Directory to where DICOM files are copied

Options:
  -p, --patient-id <PATIENT_ID>
          Patient ID (unique patient identifier)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```