//! `xdot` draw attribute parser without the graph related parts.

use nom::error::Error as NomError;

pub mod draw;
mod op_parser;
mod ops;
pub mod shapes;

pub use self::draw::Pen;
use self::shapes::Shape;

#[cfg(feature = "pyo3")]
fn try_into_shape(shape: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Shape> {
    use pyo3::prelude::*;

    if let Ok(ell) = shape.extract::<shapes::Ellipse>() {
        Ok(ell.into())
    } else if let Ok(points) = shape.extract::<shapes::Points>() {
        Ok(points.into())
    } else if let Ok(text) = shape.extract::<shapes::Text>() {
        Ok(text.into())
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(format!(
            "Cannot convert object of type {} to Shape",
            shape.get_type().name()?
        )))
    }
}

/// A [Shape] together with a [Pen].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, module = "xdot_rs"))]
pub struct ShapeDraw {
    // #[pyo3(get, set)] not possible with cfg_attr
    pub pen: Pen,
    pub shape: Shape,
}
#[cfg(feature = "pyo3")]
#[pyo3::pymethods]
impl ShapeDraw {
    #[new]
    fn new(shape: &pyo3::Bound<'_, pyo3::PyAny>, pen: Pen) -> pyo3::PyResult<Self> {
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
    fn get_shape<'py>(
        &self,
        py: pyo3::Python<'py>,
    ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        use pyo3::IntoPyObjectExt;
        match &self.shape {
            Shape::Ellipse(e) => e.clone().into_bound_py_any(py),
            Shape::Points(p) => p.clone().into_bound_py_any(py),
            Shape::Text(t) => t.clone().into_bound_py_any(py),
        }
    }
    #[setter]
    fn set_shape(&mut self, shape: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<()> {
        self.shape = try_into_shape(shape)?;
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[test]
fn cmp_equal() {
    use super::*;
    use pyo3::{IntoPyObjectExt, prelude::*};

    pyo3::prepare_freethreaded_python();

    let ellip = shapes::Ellipse {
        x: 0.,
        y: 0.,
        w: 0.,
        h: 0.,
        filled: true,
    };
    Python::with_gil(|py| {
        let a = ShapeDraw::new(&ellip.clone().into_bound_py_any(py)?, Pen::default())?;
        let b = ShapeDraw::new(&ellip.clone().into_bound_py_any(py)?, Pen::default())?;
        assert!(
            a.into_bound_py_any(py)?
                .getattr("__eq__")?
                .call1((b,))?
                .extract::<bool>()?
        );
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
    use pyo3::{PyErr, exceptions::PyValueError};

    parse(input).map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
}
