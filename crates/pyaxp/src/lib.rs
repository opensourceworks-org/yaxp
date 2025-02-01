use pyo3::prelude::*;
use yaxp_common::xsdp::parser::parse_file;


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn parse_xsd_to_json(py: Python, xsd_file: &str) -> PyResult<PyObject> {
    let result = parse_file(xsd_file);

    match result {
        Ok(schema) => {
            match schema.into_pyobject(py) {
                Ok(py_schema) => Ok(py_schema.into()),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e))),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyaxp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_xsd_to_json, m)?)?;
    Ok(())
}
