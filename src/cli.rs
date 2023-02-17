use clap::Parser;
use crate::formats::{Graph6Format, OutputFormat};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

    /// The path to the file to read
    #[clap(short, long)]
    pub input: String,

    /// The path to the file to write
    #[clap(short, long)]
    pub output: Option<String>,

    /// The graph6 format to use
    #[clap(short = 'f', long, default_value = "graph")]
    pub iformat: Graph6Format,

    /// The output format to use
    #[clap(short = 'F', long, default_value = "adjmat")]
    pub oformat: OutputFormat,

    /// Number of graphs to write
    #[clap(short, long)]
    pub count: Option<usize>,

    /// Number of graphs to skip
    #[clap(short, long)]
    pub skip: Option<usize>,

}
