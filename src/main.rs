mod cli;
mod formats;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use formats::{Graph6Format, OutputFormat};
use graph6_rs::{GraphConversion, IOError, WriteGraph};

/// A trait for graphs that can be processed by this program.
trait ProcessGraph: GraphConversion + WriteGraph {}
impl ProcessGraph for graph6_rs::Graph {}
impl ProcessGraph for graph6_rs::DiGraph {}

fn read_file(
    path: &str,
    iformat: Graph6Format,
    oformat: OutputFormat,
    count: Option<usize>,
    skip: Option<usize>,
) -> Result<()> {
    let buffer = File::open(path).map(BufReader::new)?;
    process_buffer(buffer, iformat, oformat, count, skip)?;
    Ok(())
}

fn read_stdin(
    iformat: Graph6Format,
    oformat: OutputFormat,
    count: Option<usize>,
    skip: Option<usize>,
) -> Result<()> {
    let stdin = std::io::stdin();
    let buffer = BufReader::new(stdin.lock());
    process_buffer(buffer, iformat, oformat, count, skip)?;
    Ok(())
}

fn read_graph(repr: &str, format: Graph6Format) -> Result<Box<dyn ProcessGraph>, IOError> {
    match format {
        Graph6Format::Auto => {
            if repr.starts_with('&') {
                read_graph(repr, Graph6Format::Digraph)
            } else if repr.starts_with(':') {
                read_graph(repr, Graph6Format::Sparse6)
            } else if repr.starts_with(';') {
                read_graph(repr, Graph6Format::IncSparse6)
            } else if repr.starts_with("0") || repr.starts_with("1") {
                read_graph(repr, Graph6Format::Flat)
            } else {
                read_graph(repr, Graph6Format::Graph)
            }
        }
        Graph6Format::Graph => {
            let g = graph6_rs::Graph::from_g6(repr)?;
            Ok(Box::new(g))
        }
        Graph6Format::Digraph => {
            let g = graph6_rs::DiGraph::from_d6(repr)?;
            Ok(Box::new(g))
        },
        Graph6Format::Flat | Graph6Format::Flatd => {
            let adj = repr.chars()
                .map(|c| {
                    assert!(c == '1' || c == '0', "Invalid character in adjacency matrix: {}", c);
                    c
                })
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>();
            match format {
                Graph6Format::Flat => {
                    let g = graph6_rs::Graph::from_adj(&adj)?;
                    Ok(Box::new(g))
                },
                Graph6Format::Flatd => {
                    let g = graph6_rs::DiGraph::from_adj(&adj)?;
                    Ok(Box::new(g))
                },
                _ => unreachable!(),
            }
        },
        _ => {
            unimplemented!("Sparse graphs are not supported yet.")
        }
    }
}

fn process_buffer<B: BufRead>(
    buffer: B,
    iformat: Graph6Format,
    oformat: OutputFormat,
    count: Option<usize>,
    skip: Option<usize>,
) -> Result<()> {
    let lines = buffer.lines();
    let mut idx = 0;
    let mut n_graphs = 0;
    for line in lines {
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
                },
                OutputFormat::Flat => {
                    let flat = graph.to_flat();
                    println!("{}", flat);
                },
                OutputFormat::Nauty => {
                    let d6 = graph.write_graph();
                    println!("{}", d6);
                },
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    if let Some(input) = args.input {
        read_file(&input, args.iformat, args.oformat, args.count, args.skip)?;
    } else {
        read_stdin(args.iformat, args.oformat, args.count, args.skip)?;
    }
    Ok(())
}
