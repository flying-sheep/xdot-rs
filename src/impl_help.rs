#[macro_export]
macro_rules! impl_richcmp_eq {
    () => {
        fn __richcmp__(
            &self,
            other: &Self,
            op: pyo3::pyclass::CompareOp,
            py: pyo3::Python,
        ) -> pyo3::PyObject {
            use pyo3::pyclass::CompareOp::*;
            use pyo3::IntoPy;

            match op {
                Eq => (self == other).into_py(py),
                Ne => (self != other).into_py(py),
                _ => py.NotImplemented(),
            }
        }
    };
}

pub(crate) use impl_richcmp_eq;
