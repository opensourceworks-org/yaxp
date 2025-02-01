use pyo3::prelude::*;
use yaxp_common::xsdp::parser::parse_file;


#[pyfunction]
fn parse_xsd(py: Python, xsd_file: &str) -> PyResult<PyObject> {
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

// main entrypoint for python module
#[pymodule]
fn pyaxp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_xsd, m)?)?;
    Ok(())
}
