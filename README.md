# SHACL Validation Tool

This Rust tool validates RDF data against a SHACL (Shapes Constraint Language) file. It also allows for the loading of ontologies and external terminologies for use in the validation process.

## Usage

To use this tool, follow these steps:

1. Clone the repository
2. Install Rust and Cargo
3. Run `cargo build --release`
4. Execute the tool with the following command: `./shacl_validation --input [input_ttl_file] --shacl [shacl_ttl_file] --ontologies [ontologies_ttl_files_path] --terminologies [external_terminologies_ttl_files_path] --report [output_report_file_path] --verbose [true/false]`

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

## Contribution

Contributions to shacl_validation are welcome! If you encounter any issues or have any feature requests, please feel free to open an issue on the GitHub repository. 

## License 

This project is licensed under the MIT License - see the LICENSE file for details.


