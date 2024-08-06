# dcm_sort

## Description

Sorts DICOM files into subdirectories based on a set of tags.

Template of the directory structure created in the output directory:

```less
<patient ID>/<study>/<serie>/<serie number>/<modality>

where 

- patient ID: unique identifier of a patient
- study: <STUDY_DESCRIPTION || STUDY_INSTANCE_UID || 'STUDY_UID_UNKNOWN'>
- serie: <SERIES_DESCRIPTION || SERIES_INSTANCE_UID || 'SERIES_UID_UNKNOWN'>
- serie number: <SERIES_NUMBER || SERIES_NUMBER_UNKNOWN'>
- modality: <MODALITY || MODALITY_UNKNOWN'>
```

## Build

```shell
cargo build --release
```

## Usage

```shell
dcm_sort --help
A command line interface (CLI) application to sort DICOM files into a set of subdirectories.

The DICOM data will be sorted by:
- Patient ID
- Study ID
- Series ID
- Series Number
- Modality


Usage: dcm_sort.exe [OPTIONS] --input <DIR> --output <DIR>

Options:
  -i, --input <DIR>
          Directory from where DICOM files are read

  -o, --output <DIR>
          Directory to where DICOM files are copied to

      --debug
          Enable logging at DEBUG level

      --trace
          Enable logging at TRACE level

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
