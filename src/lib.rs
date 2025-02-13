use pyo3::prelude::*;

#[pyfunction]
fn mean(numbers: Vec<f64>) -> f64 {
    let sum: f64 = numbers.iter().sum();
    let len = numbers.len() as f64;
    if len == 0.0 {
        0.0
    } else {
        sum/len
    }
}

#[pymodule]
fn ractors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    Ok(())
}
