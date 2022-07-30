use amazing_calc::my_calc;
use cpython::{py_fn, py_module_initializer, PyResult, Python};

extern crate amazing_calc;

py_module_initializer!(amazing_calc, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;

    // This is a workaround for the clippy warning:
    // Currently py_fn! macro does manual_strip internally.
    #[allow(clippy::manual_strip)]
    m.add(
        py,
        "my_calc",
        py_fn!(py, my_calc_py(a: i64, b: i64, c: i64)),
    )?;
    Ok(())
});

fn my_calc_py(_: Python, a: i64, b: i64, c: i64) -> PyResult<String> {
    let out: String = my_calc(a, b, c);
    Ok(out)
}
