use pyo3::prelude::*;

#[pyfunction]
fn mean(numbers: Vec<f64>) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
 
    let sum: f64 = numbers.iter().sum();
    sum / (numbers.len() as f64)
}

#[pyfunction]
fn median(mut numbers: Vec<f64>) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = numbers.len();

    if n % 2 == 0 {
        let mid1 = numbers[(n / 2) - 1];
        let mid2 = numbers[n / 2];
        (mid1 + mid2) / 2.0
    } else {
        numbers[n / 2]
    }
}

#[pyfunction]
fn std_dev(numbers: Vec<f64>) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }

    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;

    let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64;

    variance.sqrt()
}


#[pymodule]
fn ractors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    m.add_function(wrap_pyfunction!(median, m)?)?;
    m.add_function(wrap_pyfunction!(std_dev, m)?)?;
    Ok(())
}
