# convg

a tool to convert directed and undirected graphs from NAUTY and Traces into
adjacency, dot, and net files. (**CONV**ert **G**raph)

Inspired by the `NAUTY` gtools suite - specifically the `listg` function but
extending the functionality for directed graphs.

This should be able to determine what format your graphs are in, but you can
fix the format beforehand with the `-f` flag.

## Install

```bash
cargo install convg
```

## Usage

```bash

# Converting from a `digraph6` file to a `DOT` file
convg -i my_directed_graph.g6

# Piping from undirected `graph6` file to adjacency matrices
geng -c 4 | convg -F adjmat

# Piping from directed `graph6` file to .NET
geng -c 4 | watercluster2 Z | convg -F net

# Skipping the first 3 graphs
convg -i my_directed_graph.g6 -s 3

# Only writing the first graph
convg -i my_directed_graph.g6 -c 1

# Writing graphs 10-13
convg -i my_directed_graph.g6 -s 10 -c 3
```
