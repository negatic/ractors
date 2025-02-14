use csv::ReaderBuilder;
use pyo3::prelude::*;
use std::fs::File;
use std::io::BufReader;

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

#[pyclass]
struct CSV {
    file_path: String,
    has_headers: bool,
    delimiter: u8,
}

#[pymethods]
impl CSV {
    #[new]
    fn new(file_path: String, has_headers: bool, delimiter: u8) -> Self {
        CSV {
            file_path,
            has_headers,
            delimiter,
        }
    }

    fn open(&self, _py: Python) -> PyResult<Vec<Vec<String>>> {
        let file = File::open(&self.file_path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        let mut reader = ReaderBuilder::new()
            .has_headers(self.has_headers)
            .delimiter(self.delimiter)
            .from_reader(BufReader::new(file));

        let mut records = Vec::new();
        for result in reader.records() {
            let record =
                result.map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            records.push(record.iter().map(String::from).collect());
        }

        Ok(records)

    }

    fn columns(&self, _py: Python) -> PyResult <Vec<String>> {
        let file = std::fs::File::open(&self.file_path)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        
        let mut reader = ReaderBuilder::new()
        .has_headers(self.has_headers)
        .delimiter(self.delimiter)
        .from_reader(BufReader::new(file));

        let headers = reader.headers()
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        
        Ok(headers.iter().map(String::from).collect())
    }
}

#[pymodule]
fn ractors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    m.add_function(wrap_pyfunction!(median, m)?)?;
    m.add_function(wrap_pyfunction!(std_dev, m)?)?;
    m.add_class::<CSV>()?;
    Ok(())
}
