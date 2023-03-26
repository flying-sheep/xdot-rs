//! `xdot` draw attribute parser without the graph related parts.

use nom::error::Error as NomError;

pub mod draw;
mod op_parser;
mod ops;
pub mod shapes;

pub use self::draw::Pen;
use self::shapes::Shape;

#[cfg(feature = "pyo3")]
fn try_into_shape(shape: &pyo3::PyAny) -> pyo3::PyResult<Shape> {
    if let Ok(ell) = shape.extract::<shapes::Ellipse>() {
        Ok(ell.into())
    } else if let Ok(points) = shape.extract::<shapes::Points>() {
        Ok(points.into())
    } else if let Ok(text) = shape.extract::<shapes::Text>() {
        Ok(text.into())
    } else {
        shape.extract::<shapes::PyShape>().map(|s| s.0)
    }
}

/// A [Shape] together with a [Pen].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct ShapeDraw {
    // #[pyo3(get, set)] not possible with cfg_attr
    pub pen: Pen,
    pub shape: Shape,
}
#[cfg(feature = "pyo3")]
#[pyo3::pymethods]
impl ShapeDraw {
    #[new]
    fn new(shape: &pyo3::PyAny, pen: Pen) -> pyo3::PyResult<Self> {
        let shape = try_into_shape(shape)?;
        Ok(ShapeDraw { shape, pen })
    }
    #[getter]
    fn get_pen(&self) -> Pen {
        self.pen.clone()
    }
    #[setter]
    fn set_pen(&mut self, pen: Pen) {
        self.pen = pen;
    }
    #[getter]
    fn get_shape(&self) -> shapes::PyShape {
        shapes::PyShape(self.shape.clone())
    }
    #[setter]
    fn set_shape(&mut self, shape: shapes::PyShape) {
        self.shape = shape.0;
    }
    fn __richcmp__(
        &self,
        other: pyo3::PyRef<ShapeDraw>,
        op: pyo3::basic::CompareOp,
    ) -> pyo3::Py<pyo3::PyAny> {
        use pyo3::pyclass::CompareOp::*;
        use pyo3::IntoPy;

        let py = other.py();
        match op {
            Eq => (*self == *other).into_py(py),
            Ne => (*self != *other).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

#[cfg(feature = "pyo3")]
#[test]
fn cmp_equal() {
    use super::*;
    use pyo3::prelude::*;
    use pyo3::pyclass::CompareOp;

    pyo3::prepare_freethreaded_python();

    let ellip = shapes::Ellipse {
        x: 0.,
        y: 0.,
        w: 0.,
        h: 0.,
        filled: true,
    };
    Python::with_gil(|py| {
        let a = ShapeDraw::new(ellip.clone().into_py(py).as_ref(py), Pen::default())?;
        let b = ShapeDraw::new(ellip.clone().into_py(py).as_ref(py), Pen::default())?;
        assert!(a
            .__richcmp__(PyCell::new(py, b)?.try_into()?, CompareOp::Eq)
            .extract::<bool>(py)?);
        Ok::<(), PyErr>(())
    })
    .unwrap();
}

/// Parse an `xdot` draw attribute (as defined [here](https://graphviz.org/docs/outputs/canon/#xdot)).
/// Returns a vector of stateless drawing operations defining shape and style of the drawn node, edge, or label.
pub fn parse(input: &str) -> Result<Vec<ShapeDraw>, NomError<&str>> {
    use ops::Op::*;
    let mut pen = Pen::default();
    let mut shape_draws = vec![];
    for op in op_parser::parse(input)? {
        match op {
            DrawShape(shape) => shape_draws.push(ShapeDraw {
                pen: pen.clone(),
                shape,
            }),
            SetFontCharacteristics(fc) => pen.font_characteristics = fc,
            SetFillColor(color) => pen.fill_color = color,
            SetPenColor(color) => pen.color = color,
            SetFont { size, name } => {
                pen.font_size = size;
                pen.font_name = name;
            }
            SetStyle(style) => pen.line_style = style,
            ExternalImage(_) => todo!("conversion of external image op"),
        }
    }
    Ok(shape_draws)
}

#[cfg(feature = "pyo3")]
#[pyo3::pyfunction]
#[pyo3(name = "parse")]
pub fn parse_py(input: &str) -> pyo3::PyResult<Vec<ShapeDraw>> {
    use pyo3::{exceptions::PyValueError, PyErr};

    parse(input).map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
}
