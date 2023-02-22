use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use shacl::validator::{ValidationReport, ValidationOptions};
use shacl::data_model::{Graph, GraphRef};
use shacl::shacl::Shacl;
use shacl::parser::{parse_graph_from_file, parse_graph_from_str};

fn shacl_validation() -> Result<(), Box<dyn Error>> {
    // Accept input from the user
    let input_file = std::env::args().nth(1).unwrap();
    let shacl_file = std::env::args().nth(2).unwrap();
    let external_files = std::env::args().skip(3);
    let output_file = std::env::args().nth(4).unwrap();
    let verbose = std::env::args().nth(5).unwrap_or_default();
    let verbose_mode = verbose.parse().unwrap_or(false);

    // Load the input TTL file into a Graph
    let input_graph = parse_graph_from_file(&input_file)?;

    // Load the SHACL TTL file into a Graph
    let shacl_graph = parse_graph_from_file(&shacl_file)?;

    // Load any external terminology files into a Graph
    let mut external_graphs = Vec::new();
    for file in external_files {
        let graph = parse_graph_from_file(&file)?;
        external_graphs.push(graph);
    }

    // Combine all the graphs into a single Graph
    let graph = Graph::new(None);
    let graph_ref: GraphRef = graph.borrow();

    graph_ref.merge(&input_graph)?;
    graph_ref.merge(&shacl_graph)?;

    for g in &external_graphs {
        graph_ref.merge(&g)?;
    }

    // Create a SHACL validator
    let shacl = Shacl::new(&graph)?;

    // Validate the input against the SHACL shapes
    let validation_options = ValidationOptions::default();
    let report = shacl.validate(&graph, &validation_options)?;

    // Generate a validation report and write it to the output file
    let mut report_file = File::create(output_file)?;
    let report_str = report.to_turtle_string()?;
    write!(report_file, "{}", report_str)?;

    if verbose_mode {
        println!("Validation completed successfully! Validation report written to {}", output_file);
    }

    Ok(())
}
