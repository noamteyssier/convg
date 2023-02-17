use clap::ValueEnum;

#[derive(Default, Debug, Copy, Clone, ValueEnum)]
pub enum Graph6Format {
    /// The default undirected format
    #[default]
    Graph,
    /// The digraph format
    Digraph,
    /// The sparse6 format
    Sparse6,
}

#[derive(Default, Debug, Copy, Clone, ValueEnum)]
pub enum OutputFormat {
    /// The default adjacency matrix format
    #[default]
    Adjmat,
    /// The DOT format
    DOT,
    /// The Pavek NET format
    NET,
}
