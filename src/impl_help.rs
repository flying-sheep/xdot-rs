#[macro_export]
macro_rules! impl_bitflags_accessors {
    ($cls: path, $( $flag: ident ),+ $(,)?) => {
        paste::paste! {
            #[cfg(feature = "pyo3")]
            #[pyo3::pymethods]
            impl $cls {
                $(
                    #[getter]
                    fn [< get_ $flag >](&self) -> bool {
                        self.contains($cls::[< $flag:upper >])
                    }
                    #[setter]
                    fn [< set_ $flag >](&mut self, value: bool) {
                        self.set($cls::[< $flag:upper >], value);
                    }
                )+
            }
        }
    };
}
