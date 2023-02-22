# SHACL Validation Tool

The shacl_validation.rs tool is a command-line tool written in Rust that performs SHACL validation on a given RDF TTL data file using a SHACL TTL file and external terminology TTL files. The tool loads the ontology and external terminology files into the file system for optimized memory and performance.

## Prerequisites

To run the shacl_validation.rs tool, you need to have Rust installed on your system. You can download Rust from the official website: https://www.rust-lang.org/tools/install

## Usage

To use this tool, follow these steps:

1. Clone the repository
2. Install Rust and Cargo
3. Run `cargo build --release`
4. Execute the tool with the following command: 

`./shacl_validation --input [input_ttl_file] --shacl [shacl_ttl_file] --ontologies [ontologies_ttl_files_path] --terminologies [external_terminologies_ttl_files_path] --report [output_report_file_path] --verbose [true/false]`

### Arguments

* `--input`: The input RDF TTL file to validate.
* `--shacl`: The SHACL TTL file containing the validation rules.
* `--ontologies`: (Optional) The path to the directory containing any ontologies to be loaded.
* `--terminologies`: (Optional) The path to the directory containing any external terminologies to be loaded.
* `--report`: (Optional) The output file path for the validation report. Defaults to "report.ttl".
* `--verbose`: (Optional) A boolean flag to enable or disable verbose mode. Defaults to "false".

## Example

Here's an example of how to run the tool:

```shell
./shacl_validation --input example_data.ttl --shacl example_shapes.ttl --ontologies ontologies/ --terminologies terminologies/ --report report.ttl --verbose true
```

## Contribution

Contributions to shacl_validation are welcome! If you encounter any issues or have any feature requests, please feel free to open an issue on the GitHub repository. 

## License 

This project is licensed under the MIT License - see the LICENSE file for details.


