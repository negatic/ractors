use pyo3::prelude::*;

#[pyfunction]
fn mean(numbers: Vec<f64>) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
 
    let sum: f64 = numbers.iter().sum();
    sum / (numbers.len() as f64)
}

#[pymodule]
fn ractors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    Ok(())
}
