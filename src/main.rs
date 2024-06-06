use std::fs;
use std::fs::File;
mod safe_paths {
    mod edge;
    mod graph;
    mod flow;
    mod uniqueness;
    pub mod safe_paths;
}
mod node_to_arc_centric {
    pub mod node_to_arc;
}
use crate::safe_paths::safe_paths::safe_paths;
use crate::node_to_arc_centric::node_to_arc::node_to_arc_centric_dbg_with_memory_meter;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use clap::Parser;
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use crate::memory_meter::MemoryMeter;
mod memory_meter;
 


#[derive(Parser, Debug)]
struct Cli {
    /// The input file containing an arc-centric de Bruijn graph.
    /// The file should be an edgelist with the number of nodes on the first row, then one row for each edge containing the starting node, end node, weight and sequence; each separated by one space.
    #[clap(long)]
    input: PathBuf,

    /// The k-mer size used to generate the de Bruijn graph.
    #[clap(short)]
    k: usize,

    /// The threshold for safety.
    #[clap(short)]
    threshold: i64,

    /// The output file where the arc-centric de Bruijn graph should be written to.
    #[clap(long)]
    output: PathBuf,

    /// The desired log level.
    #[clap(long, default_value = "Info")]
    log_level: LevelFilter,
}

pub fn initialise_logging(log_level: LevelFilter) {
    CombinedLogger::init(vec![TermLogger::new(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    info!("Logging initialised successfully");
}


pub fn new_file(path: &PathBuf) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(&1234_u32.to_be_bytes())?;
    Ok(())
}


fn main() {
    let mut meter = MemoryMeter::new();
    info!("Memory meter built successfully");
    
    let cli = Cli::parse();
    initialise_logging(cli.log_level);
    info!(
        "Loading graph from {:?} with k = {} and writing to {:?}",
        cli.input, cli.k, cli.output
    );

    let mut input = BufReader::new(File::open(&cli.input).unwrap());

    // Transform the node-centric graph in the input into an edge-centric graph
    let input_file_stem = cli.input.file_stem();
    let mut string_stem = String::new();
     if let Some(osstr_stem) = input_file_stem {
        if let Some(str_stem) = osstr_stem.to_str() {
            string_stem = str_stem.to_owned();
        } else {
            info!("Error: failed to convert input file stem to string");
        }
    } else {
        info!("Error: failed to convert input file stem to string");
    }
    let edge_centric_path = PathBuf::from("src/edge_centric_graphs/".to_string() + &string_stem + ".edgelist");
    // let mut _empty_file = new_file(&edge_centric_path); // File::create(edge_centric_path)?; // fs::write(&edge_centric_path, "AAAA");
    info!("Path: {}", edge_centric_path.display());
    info!("Input: {}", cli.input.display());
    info!("Output: {}", cli.output.display());
    let mut edge_centric_file = BufWriter::new(File::create(&edge_centric_path).unwrap());
    node_to_arc_centric_dbg_with_memory_meter(cli.k, &mut input, &mut edge_centric_file, Some(&mut meter));

    meter.report();
    info!("Edge-centric graph built successfully!");


    // Compute safe paths from edge-centric graph
    let mut edge_centric_string_path = String::new();
    if let Some(edge_cent_str_path) = edge_centric_path.to_str() {
        edge_centric_string_path = edge_cent_str_path.to_owned();
    }
    else{
        info!("Error: Failed to convert path");
    }
    let mut output = BufWriter::new(File::create(&cli.output).unwrap());
    let safe_paths = safe_paths(&edge_centric_string_path, cli.k, cli.threshold, Some(&mut meter));

    info!("Safe paths calculated");
    meter.report();



    let mut counter = 0;
    for sequence in &safe_paths {
        writeln!(output, ">Path_{}", counter).unwrap();
        writeln!(output, "{} ", sequence).unwrap();
        counter += 1;
    }
    info!("Safe paths written to file");
    meter.report();
}



