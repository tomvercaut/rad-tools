# dcm_study_series_uid

A command-line tool to extract the Study Instance UID and Series Instance UID from a DICOM file.

## Description

`dcm_study_series_uid` is a command-line interface (CLI) utility that reads a DICOM file, extracts its Study Instance UID (`(0020,000D)`) and Series Instance UID (`(0020,000E)`), and outputs the identifiers in JSON format.

For optimal performance, the application reads the DICOM dataset only up to the tags in group `0020`, avoiding the overhead of parsing large payloads such as pixel data.

## Build

To build the application using Cargo:

```shell
cargo build --release
```

Or build only this specific package:

```shell
cargo build -p rad-tools-dcm-study-series-uid --release
```

## Usage

```shell
dcm_study_series_uid --help
A command line interface (CLI) application to extract the study and serie instance UID from a DICOM file

Usage: dcm_study_series_uid [FILE]

Arguments:
  [FILE]  Filename to a DICOM file, if not specified, the filename will read from standard input

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Examples

#### Pass a filename as an argument

```shell
dcm_study_series_uid path/to/file.dcm
```

#### Read a filename from standard input

If no file argument is provided, the tool reads the path from `stdin`:

```shell
echo "path/to/file.dcm" | dcm_study_series_uid
```

#### Example Output

The output is formatted as pretty-printed JSON:

```json
{
  "study": "1.2.840.10008.1.2.3.4.5.6.7",
  "series": "1.2.840.10008.1.2.3.4.5.6.8"
}
```
