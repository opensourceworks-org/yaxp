use std::convert::Infallible;
use pyo3::prelude::*;
use pyo3::types::{PyNone, PyString};
use yaxp_common::xsdp::parser::parse_file;


#[pyfunction]
fn parse_xsd(py: Python, xsd_file: &str, format: &str) -> PyResult<PyObject> {
    let result = parse_file(xsd_file);

    match result {
        Ok(schema) => {

            match format {
                "json" => {
                    match schema.into_pyobject(py) {
                        Ok(py_schema) => Ok(py_schema.into()),
                        Err(e) => Err(e),
                    }

                }
                "arrow" => {
                    match schema.to_arrow() {

                        Ok(arrow) => {
                            match arrow.to_string().into_pyobject(py) {
                                Ok(py_arrow) => Ok(py_arrow.into()),

                                _ => {Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Error converting to arrow"))}
                            }
                        }

                        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e))),
                    }
                }
                "spark" => {
                    match schema.to_spark() {
                        Ok(spark) => {
                            match spark.to_json().unwrap().into_pyobject(py) {
                                Ok(py_spark) => Ok(py_spark.into()),

                                _ => {Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Error converting to spark"))}
                            }
                        }
                        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e))),
                    }
                }
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid format: {}", format))),
            }

        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e))),
    }
    // match result {
    //     Ok(schema) => {
    //         match schema.into_pyobject(py) {
    //             Ok(py_schema) => Ok(py_schema.into()),
    //             Err(e) => Err(e),
    //         }
    //     }
    //     Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e))),
    // }
}

// main entrypoint for python module
#[pymodule]
fn pyaxp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_xsd, m)?)?;
    Ok(())
}
