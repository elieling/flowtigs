# Flowtigs

An algorithm for calculating flowtigs in a De Bruijn graph of DNA reads in metagenomes.

## Input

The input file should be an edgelist representing an arc-centric De Bruijn graph. The format of the edgelist should be as follows:
- Each edge is represented by a row with 4 values separated by a whitespace.
- The values, in this order, should be:
	1. Starting node
	2. Ending node
	3. Weight
	4. String sequence

## Output

The output of this algorithm is a FASTA file, which contains the safe maximal string sequences named by an index from 0 to `<total number of sequences> - 1`.

## Running instructions

The algorithm is ran with the following command, assuming that [Rust](https://rustup.rs/) is installed:

`cargo run --release -- -k {k} --input '{arc_centric_dbg}' --output '{safe_paths}' 2>&1 | tee -a '{log}'`

where:
- {k} is the size of the k-mers used in the De Bruijn graph.
- {arc_centric_dbg} is the input edgelist.
- {safe_paths} is the desired path to the output file.
- {log} is the desired path to the log file.

The algorithm can also be ran without a log file with the following command:

`cargo run --release -- -k {k} --input '{arc_centric_dbg}' --output '{safe_paths}'`

## Installation

First, install [Rust](https://rustup.rs/) if not yet installed.

### Installing Rust on Linux or macOS

Run on terminal

`$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

### Installing Rust on Windows

Follow this [link](https://www.rust-lang.org/tools/install) and follow the instructions

### Installing flowtigs

Create a new project with

`cargo new hello_cargo
cd hello_cargo`

or navigate to your project directory, then run

`cargo add flowtigs` 

or add `flowtigs = "1.0.1"` to your Cargo.toml file.
