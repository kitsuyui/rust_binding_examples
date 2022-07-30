use pyo3::prelude::*;


#[pyfunction]
fn my_calc(a: i64, b: i64, c: i64) -> PyResult<String> {
    Ok(amazing_calc::my_calc(a, b, c).to_string())
}


#[pymodule]
fn amazing_calc(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(my_calc, m)?)?;
    Ok(())
}
