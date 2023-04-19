[![CI](https://github.com/flying-sheep/xdot-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/flying-sheep/xdot-rust/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/xdot)](https://docs.rs/xdot/latest/xdot/)
[![Crates.io](https://img.shields.io/crates/v/xdot)](https://crates.io/crates/xdot)
[![PyPI](https://img.shields.io/pypi/v/xdot-rs)](https://pypi.org/project/xdot-rs/)

xdot
====

The main function of this package is `parse`.
It parses node/edge attributes on graphviz graphs created by [`xdot`](https://graphviz.org/docs/attr-types/xdot/) into drawable shapes.

```rust
use xdot::{parse, ShapeDraw};
let shapes: Vec<ShapeDraw> = parse("c 7 -#ff0000 p 4 4 4 36 4 36 36 4 36");
```

Each `ShapeDraw` struct contains a `shape` with geometry and a `pen` with drawing attributes (such as color, line style, and font).
If you have the `layout` feature active, you can also use `layout_and_draw_graph` (and `draw_graph`):

```rust
use graphviz_rust::parse;
use graphviz_rust::dot_structures::Graph;
use xdot::{layout_and_draw_graph, ShapeDraw};

let graph: Graph = parse("graph { a -- b}").unwrap();
let shapes: Vec<ShapeDraw> = layout_and_draw_graph(graph).unwrap();
```

Release process
---------------

To be automated. Currently:

```console
$ # update version
$ $EDITOR Cargo.toml  # update version
$ cargo build  # update lockfile
$ git add Cargo.toml Cargo.lock && git commit && git tag v??????
$ # publish
$ cargo publish
$ docker run --rm -v "$PWD:/io" ghcr.io/pyo3/maturin:v1.0.0-beta.7 build --release -i python3.8 -i python3.9 -i python3.10 -i python3.11
$ twine upload target/wheels/*
```
