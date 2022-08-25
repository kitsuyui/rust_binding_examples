use amazing_calc as rust_amazing_calc;
use pyo3::prelude::*;

#[pyfunction]
fn my_calc(a: i64, b: i64, c: i64) -> PyResult<String> {
    Ok(rust_amazing_calc::my_calc(a, b, c))
}

#[pymodule]
#[pyo3(name = "amazing_calc")]
fn python_amazing_calc(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(my_calc, m)?)?;
    Ok(())
}
