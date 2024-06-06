use std::fs::File;
mod safe_paths {
    mod edge;
    mod graph;
    mod flow;
    mod uniqueness;
    pub mod safe_paths;
}
use crate::safe_paths::safe_paths::safe_paths;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use clap::Parser;
use std::io::Write;
use std::io::BufWriter;
use std::path::PathBuf;
use crate::memory_meter::MemoryMeter;
mod memory_meter;
 


#[derive(Parser, Debug)]
struct Cli {
    /// The input file containing an arc-centric de Bruijn graph.
    /// The file should be an edgelist with the number of nodes on the first row, then one row for each edge containing the starting node, end node, weight and sequence; each separated by one space.
    #[clap(long)]
    input: String,

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






fn main() {
    let mut meter = MemoryMeter::new();
    info!("Memoery meter built successfully");
    // Choose the file you want to use
 
    let cli = Cli::parse();
    initialise_logging(cli.log_level);
    info!(
        "Loading graph from {:?} with k = {} and writing to {:?}",
        cli.input, cli.k, cli.output
    );
    let mut output = BufWriter::new(File::create(&cli.output).unwrap());
    meter.report();
    let safe_paths = safe_paths(&cli.input, cli.k, cli.threshold, Some(&mut meter));

    info!("Safe paths calculated");
    meter.report();


    // println!("\n++++++++ Then, the safe paths as final unique strings: ++++++++");
    let mut counter = 0;
    for sequence in &safe_paths {
        writeln!(output, ">Path_{}", counter).unwrap();
        writeln!(output, "{} ", sequence).unwrap();
        counter += 1;
    }
    info!("Safe paths written to file");
    meter.report();
}



