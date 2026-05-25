#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_cfg))]

//! Parse and draw [`xdot`](https://graphviz.org/docs/attr-types/xdot/) shapes.
//!
//! ## Example
//! ```rust
//! use xdot::parse;
//! let shapes = parse("c 7 -#ff0000 p 4 4 4 36 4 36 36 4 36");
//! ```
//!
//! ## Feature flags
#![cfg_attr(all(doc, feature = "document-features"), doc = document_features::document_features!())]

#[macro_use]
mod impl_help;
#[cfg(feature = "layout")]
mod layout;
mod xdot_parse;

#[cfg(feature = "layout")]
pub use self::layout::{LayoutError, draw_graph, layout_and_draw_graph};
pub use self::xdot_parse::{ShapeDraw, draw, parse, shapes};

/// Known node/edge attribute names holding `xdot` draw instructions that [parse] can handle.
///
/// If the [feature flag](crate#feature-flags) `layout` is active, this is by [draw_graph] when traversing the graph.
pub static ATTR_NAMES: [&str; 6] = [
    "_draw_", "_ldraw_", "_hdraw_", "_tdraw_", "_hldraw_", "_tldraw_",
];

/// Python module TODO
#[cfg(feature = "pyo3")]
#[pyo3::pymodule]
pub mod xdot_rs {
    use pyo3::{exceptions::PyValueError, prelude::*, types::PyModule};

    #[pymodule_export]
    pub use super::{ShapeDraw, draw::draw, shapes::shapes};

    #[pyfunction]
    pub fn parse(input: &str) -> pyo3::PyResult<Vec<ShapeDraw>> {
        super::xdot_parse::parse(input).map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        Python::attach(|py| {
            let mods = py.import("sys")?.getattr("modules")?;
            mods.set_item("xdot_rs.draw", m.getattr("draw")?)?;
            mods.set_item("xdot_rs.shapes", m.getattr("shapes")?)?;
            Ok(())
        })
    }
}
