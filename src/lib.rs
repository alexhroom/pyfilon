use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use num_complex::Complex;
extern crate filon as filon_rs;

/// macro that wraps filon function as pyfunction
macro_rules! pyo3ise {
    ($fn_name:ident) => {
    #[pyfunction]
    fn $fn_name(ftab: Vec<Complex<f64>>, a: f64, b: f64, coeff: f64) -> PyResult<Complex<f64>> {
        match filon_rs::$fn_name(ftab, a, b, coeff) {
            Ok(result) => Ok(result),
            Err(error) => Err(PyValueError::new_err(error.to_string()))
            }
        }
    }
}

pyo3ise!(filon_tab_sin);
pyo3ise!(filon_tab_cos);

#[pymodule]
fn pyfilon(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(filon_tab_sin, m)?)?;
    m.add_function(wrap_pyfunction!(filon_tab_cos, m)?)?;
    Ok(())
}
