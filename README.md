# Flowtigs

An algorithm for calculating flowtigs in a De Bruijn graph of DNA reads in metagenomes.

## Installation

First, install [Rust](https://rustup.rs/) if not yet installed.

### Installing Rust on Linux or macOS

Run the following code snippet in your terminal window

```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Installing Rust on Windows

Follow this [link](https://www.rust-lang.org/tools/install) and follow the instructions to install rustup.

## Running flowtigs

Clone this project
```
git clone https://github.com/elieling/flowtigs.git
cd flowtigs
```
Run flowtigs with the folowing code in the project directory
```
flowtigs --input "<input file>" -k <k> -t <threshold> --output "<output file>"
```
where 
- `<input file>` represents the path to the input file
- `<k>` represents the desired k-value
- `<threshold>` represents the desired threshold for filtering. To run flowtigs without filtering, use threshold 0
- `<output file>` represents the path to the desired output file


## Input

The input of flowtigs should be a file in the same format as the output of [bcalm](https://github.com/GATB/bcalm) or [ggcat](https://github.com/algbio/ggcat). An example can be seen [here](https://github.com/elieling/flowtigs-data).

### Running ggcat

ggcat takes reads as input and outputs a file that can be used by flowtigs, see [example](https://github.com/elieling/flowtigs-data). Download ggcat with the following commands
```
git clone https://github.com/algbio/ggcat --recursive
cd ggcat/
git checkout a91ecc97f286b737b37195c0a86f0e11ad6bfc3b
cargo update time
cargo install --path crates/cmdline/ --locked --features "kmer-counters"
```

Then, ggcat is run with

```
ggcat build -k <k> -j <threads> -e -s <minimum multiplicity> '<input file name>' -o '<output file name>'
```
where 
- `<k>` represents the desired k-value, which is the same that will be used by flowtigs
- `<threads>` represents the number of threads on which ggcat will run
- `<minimum multiplicity>` minimum multiplicity needed for a k-mer to occur
- `<input file>` represents the path to the input file which contains the reads
- `<output file>` represents the path to the desired output file, which will be the input file for flowtigs

## Output

The output of this algorithm is a FASTA file, which contains the safe maximal string sequences named by an index from 0 to `<total number of sequences> - 1`.

