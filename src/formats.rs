use clap::ValueEnum;

#[derive(Default, Debug, Copy, Clone, ValueEnum)]
pub enum Graph6Format {
    /// The undirected format
    Graph,
    /// The digraph format
    Digraph,
    /// The sparse6 format
    Sparse6,
    /// The incremental sparse6 format
    IncSparse6,
    /// Flat undirected adjacency matrix format
    Flat,
    /// Flat directed adjacency matrix format
    Flatd,
    /// Automatically determine the format
    #[default]
    Auto,
}

#[derive(Default, Debug, Copy, Clone, ValueEnum)]
pub enum OutputFormat {
    /// The adjacency matrix format
    Adjmat,
    /// The DOT format
    #[default]
    DOT,
    /// The Pavek NET format
    NET,
    /// A flat adjacency matrix format
    Flat,
    /// The Nauty Graph6/Digraph6 format
    Nauty,
}
