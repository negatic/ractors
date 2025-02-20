use pyo3::prelude::*;
use std::{io::{BufRead, BufReader}, usize};

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
struct Dataframe {
    columns: Vec<String>,
    rows: Vec<Vec<String>>
}
#[pyclass]
struct CSV {
    file_path: String,
    delimiter: String,
}

#[pymethods]
impl Dataframe {
    #[staticmethod]
    fn default() -> Self {
        Dataframe {
            columns: Vec::new(),
            rows: Vec::new()
        }
    }

    fn headers(&self) -> &Vec<String> {
        &self.columns
    }

    fn rows(&self) -> &Vec<Vec<String>> {
        &self.rows
    }

    fn column_values_from_index(&self, column_index: usize) -> Vec<String> {
        let mut result = <Vec<String>>::new();

        for (_, row) in self.rows().iter().enumerate() {
            if row.len() > column_index {
                result.push(row[column_index].clone());
            }
        }

        result
        
    }

    fn get_column_values(&self, column_name: String) -> Vec<String> {
        match &self.columns.iter().position(|x| x == &column_name) {
            Some(index) => self.column_values_from_index(*index),
            None => return Vec::new()
        }
    }
}

#[pymethods]
impl CSV {
    #[new]
    fn new(file_path: String, delimiter: String) -> std::io::Result<Self> {
        Ok(CSV { file_path, delimiter })
    }

    fn read(&mut self) -> Dataframe {
        let file = std::fs::File::open(self.file_path.to_string()).unwrap();
        let reader = BufReader::new(file);
        let mut is_header = true;
        let mut _row_number = 0;
        let mut df = Dataframe::default();

        for row in reader.lines() {
            let row = row.expect("Failed To Read Line");

            if row.is_empty() {
                break
            } else if is_header {
                let pat = self.delimiter.to_string();
                df.columns = row.split(&pat).map(String::from).collect();
            
            } else {
                let pat = self.delimiter.to_string();
                let row_data: Vec<String> = row.split(&pat).map(String::from).collect();
                df.rows.push(row_data);
                _row_number += 1;
            }
            is_header = false;
        }
        df
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
