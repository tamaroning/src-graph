# src-graph
Cargo subcommand which shows structs dependencies.
src-graph shows only dependencies *between* user-defined structs for now.

![screenshot1](screenshot1.png)

# Requirement
- Cargo
- Graphviz (for generating an image)

# Install
From crates.io
```
$ cargo install src-graph
```

From source code
```
$ git clone https://github.com/tamaroning/src-graph.git
$ cd src-graph
$ cagro install --path .
```

# Usage
Run in your rust project
```
$ cargo src-graph
```

then generate an image from the dot file
```
$ dot -Tpng -o ./.src_graph/struct_deps.png ./.src_graph/struct_deps.dot
```
