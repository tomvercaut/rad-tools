# ls-dcm

## Description

Read minimal data from DICOM files and a print it in the terminal.

## Build

```shell
cargo build --release
```

## Usage

```shell
ls_rtplan --help
A command line interface (CLI) application for reading and listing RTPLAN DICOM files.

Application enables the user to specify the directory from which the DICOM files are read,
as well as additional options such as filtering by filename prefixes, limiting the number of displayed results,
sorting the files by last modified timestamp, and enabling logging at different levels.


Usage: ls_rtplan_r.exe [OPTIONS]

Options:
  -d, --dir <DIR>
          Directory from where DICOM files are read (recursively). If unspecified, the current directory (".") is analysed

  -p, --prefixes <PREFIXES>
          If specified, only filenames starting with a matching prefix, will be read. Specifying this will increase the performance of the application

  -l, --limit <LIMIT>
          Limit the number of displayed results

  -s, --sort
          Sort the reported data by last modified timestamp of the file

      --debug
          Enable logging at DEBUG level

      --trace
          Enable logging at TRACE level

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
