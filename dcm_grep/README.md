# dcm_grep

`dcm_grep` is a command-line interface (CLI) application to extract values of one or more DICOM elements from a single DICOM file. It uses a simple pattern syntax to navigate nested sequences and optionally prints the full element path along with values.
Typical use cases:
- Quickly print core identifiers (e.g., SOP Class UID, Patient ID)
- Extract values from nested sequences using indices, ranges, or “all” selection
- Scriptable extraction where filenames come from stdin (batch processing with shell tools)

## How it works
- You pass one or more element patterns with -e.
- Each pattern identifies a DICOM element by tag and optional selectors for sequences.
- The tool opens one DICOM file, evaluates one or more patterns, and prints the results (one value per line).
- With --recursive it searches for the pattern in nested sequences.
- With --show-path it prints “path: value” for each match.

Note: `dcm_grep` operates on a single file at a time. Use shell tools like find, xargs, or loops to process many files.
## Pattern syntax
- Tag format: 
  - (gggg,eeee) in hexadecimal, e.g. (0008,0016)
  - name, e.g. Modality, PatientName
- For sequences, you can select items with:
    - [*] all items
    - [n] zero-based index
    - [start-stop] zero-based inclusive-exclusive range

## Build

```shell
cargo build -p rad-tools-dcm-grep --release
```

## Usage

```shell
dcm_grep --help
Extract the values of one or more (nested) DICOM tags.

A pattern-based syntax is used to extract the value of the DICOM elements.

Usage: dcm_grep [OPTIONS]

Options:
  -i, --input <FILE>
          Filename to a DICOM file, if not specified, the filename will read from standard input

  -e <PATTERN>
          Use a pattern to select the DICOM elements to extract.
          
          The pattern can be one of the following:
          
          - (group,element)
          
          - (group,element)[<selector>]
          
          Where group and element are DICOM tag identifiers in hexadecimal.
          
          The optional [<selector>] can be specified on DICOM sequences:
          
          - [*] suffix matches all nested elements
          
          - [<index>] index (zero-based)
          
          - [<start>-<stop>] a range of indices (zero-based)
          
          Nested DICOM elements can be found be appending a `/` to the pattern.

          For example:

          - (0008,0016): selects the SOP Class UID

          - (3006,0010)[*]/(0020,0052): selects all the Frame Of Reference UIDs in the Referenced Frame Of Reference Sequence.

          - (3006,0010)[0]/(0020,0052): selects the Frame Of Reference UID in the first Referenced Frame Of Reference Sequence item.

          - (3006,0010)[1]/(0020,0052): selects the Frame Of Reference UID in the second Referenced Frame Of Reference Sequence item.

  -r, --recursive
          Recursively search for the pattern in nested DICOM elements

      --show-path


      --verbose


      --debug
          Enable logging at DEBUG level

      --trace
          Enable logging at TRACE level

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Examples

- Extract the SOP Class UID from a DICOM file:
```shell
dcm_grep -i input.dcm -e '(0008,0016)'
```
Output:
```
1.2.840.10008.5.1.4.1.1.2
```


- Extract the number of rows, colums and the first code value from a CTDI Phantom Type Code sequence:
```shell
dcm_grep -i ct.dcm  -e '(0028,0010)' -e '(0028,0030)' -e '(0018,9346)[*]/(0008,0100)'
```

Output:
```
512
0.55566015625\0.55566015625
11369
```

- Extract the number of rows, colums, and the first code value from a CTDI Phantom Type Code sequence using the --show-path option:
```shell
dcm_grep -i ct.dcm  -e '(0028,0010)' -e '(0028,0030)' -e '(0018,9346)[*]/(0008,0100)' --show-path
```

Output:
```
(0028,0010): 512
(0028,0030): 0.55566015625\0.55566015625
(0018,9346)[0]/(0008,0100): 113691
```

- Extract the nested Referenced SOP Class UIDs using the -r option:
```shell
dcm_grep -i ct.dcm  -e '(0008,1150)' -r 
```

Output:
```
1.2.840.10008.3.1.2.3.3
1.2.840.10008.5.1.4.1.1.2
1.2.840.10008.3.1.2.3.1
```

- Extract the number of rows, colums by name:
```shell
dcm_grep -i ct.dcm  -e 'Rows' -e 'Columns' --show-path
```

Output:
```
(0028,0010): 512
(0028,0011): 512
```
