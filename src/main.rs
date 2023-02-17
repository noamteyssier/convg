mod cli;
mod formats;

use std::{fs::File, io::{BufReader, BufRead}};

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use graph6_rs::{GraphConversion, IOError};
use formats::{Graph6Format, OutputFormat};


fn read_file(path: &str, iformat: Graph6Format, oformat: OutputFormat, count: Option<usize>, skip: Option<usize>) -> Result<()> {
    let buffer = File::open(path).map(BufReader::new)?;
    process_buffer(buffer, iformat, oformat, count, skip)?;
    Ok(())
}

fn read_graph(repr: &str, format: Graph6Format) -> Result<Box<dyn GraphConversion>, IOError> {
    match format {
        Graph6Format::Graph => {
            let g = graph6_rs::Graph::from_g6(repr)?;
            Ok(Box::new(g))
        }
        Graph6Format::Digraph => {
            let g = graph6_rs::DiGraph::from_d6(repr)?;
            Ok(Box::new(g))
        }
        Graph6Format::Sparse6 => {
            unimplemented!();
        }
    }
}

fn process_buffer<B: BufRead>(buffer: B, iformat: Graph6Format, oformat: OutputFormat, count: Option<usize>, skip: Option<usize>) -> Result<()> {
    let mut lines = buffer.lines();
    let mut idx = 0;
    let mut n_graphs = 0;
    while let Some(line) = lines.next() {
        if let Ok(record) = line {
            let repr = record.trim();
            
            // Skip empty lines
            if repr.is_empty() {
                continue;
            }
            idx += 1;

            // Skip the first `skip` graphs
            if let Some(s) = skip {
                if idx <= s {
                    continue;
                }
            }

            // Stop after `count` graphs
            if let Some(c) = count {
                if n_graphs >= c {
                    break;
                }
            }
            n_graphs += 1;

            // Read the graph
            let graph = read_graph(repr, iformat).unwrap();

            // Write the graph
            match oformat {
                OutputFormat::Adjmat => {
                    let adj_mat = graph.to_adjmat();
                    println!("{}", adj_mat);
                }
                OutputFormat::DOT => {
                    let dot = graph.to_dot(Some(idx));
                    println!("{}", dot);
                }
                OutputFormat::NET => {
                    let net = graph.to_net();
                    println!("{}", net);
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    read_file(&args.input, args.iformat, args.oformat, args.count, args.skip)?;
    Ok(())
}
