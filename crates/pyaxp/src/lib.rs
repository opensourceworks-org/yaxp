use arrow::datatypes::DataType::*;
use arrow::datatypes::{Field, IntervalUnit, Schema, TimeUnit};
use encoding_rs::{Encoding, UTF_8};
use polars::datatypes::DataType as PolarsDataType;
use pyo3::conversion::FromPyObject;
use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::PyAny;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use yaxp_core::xsdp::parser::parse_file;
use yaxp_core::xsdp::parser::TimestampOptions;

#[derive(Debug, Clone)]
enum SchemaFormat {
    Arrow,
    Avro,
    Duckdb,
    Polars,
    Json,
    JsonSchema,
    Spark,
}

impl FromStr for SchemaFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "arrow" => Ok(SchemaFormat::Arrow),
            "duckdb" => Ok(SchemaFormat::Duckdb),
            "polars" => Ok(SchemaFormat::Polars),
            "json" => Ok(SchemaFormat::Json),
            "json-schema" => Ok(SchemaFormat::JsonSchema),
            "spark" => Ok(SchemaFormat::Spark),
            "avro" => Ok(SchemaFormat::Avro),
            _ => Err(format!("Invalid format: {}, Supported formats: arrow, duckdb, json, json-schema, polars, spark. ", s)),
        }
    }
}

impl fmt::Display for SchemaFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'source> FromPyObject<'source> for SchemaFormat {
    fn extract_bound(bound: &pyo3::Bound<'source, PyAny>) -> PyResult<Self> {
        let s: String = <String as FromPyObject>::extract_bound(bound)?;

        // clippy advised to replace the closure with the function itself: `PyValueError::new_err`
        // less clutter, maybe, less readable for some
        // SchemaFormat::from_str(&s).map_err(|e| PyValueError::new_err(e))
        SchemaFormat::from_str(&s).map_err(PyValueError::new_err)
    }
}

fn convert_polars_dtype_to_pyobject(py: Python, dtype: &PolarsDataType) -> PyResult<PyObject> {
    // import python polars module
    let polars = PyModule::import(py, "polars")?;
    match dtype {
        PolarsDataType::Boolean => Ok(polars.getattr("Boolean")?.into_pyobject(py)?.into()),
        PolarsDataType::Int8 => Ok(polars.getattr("Int8")?.into_pyobject(py)?.into()),
        PolarsDataType::Int16 => Ok(polars.getattr("Int16")?.into_pyobject(py)?.into()),
        PolarsDataType::Int32 => Ok(polars.getattr("Int32")?.into_pyobject(py)?.into()),
        PolarsDataType::Int64 => Ok(polars.getattr("Int64")?.into_pyobject(py)?.into()),
        PolarsDataType::UInt8 => Ok(polars.getattr("UInt8")?.into_pyobject(py)?.into()),
        PolarsDataType::UInt16 => Ok(polars.getattr("UInt16")?.into_pyobject(py)?.into()),
        PolarsDataType::UInt32 => Ok(polars.getattr("UInt32")?.into_pyobject(py)?.into()),
        PolarsDataType::UInt64 => Ok(polars.getattr("UInt64")?.into_pyobject(py)?.into()),
        PolarsDataType::Float32 => Ok(polars.getattr("Float32")?.into_pyobject(py)?.into()),
        PolarsDataType::Float64 => Ok(polars.getattr("Float64")?.into_pyobject(py)?.into()),
        PolarsDataType::String => Ok(polars.getattr("Utf8")?.into_pyobject(py)?.into()),
        PolarsDataType::Decimal(precision, scale) => {
            let decimal_cls = polars.getattr("Decimal")?;
            // calling the Decimal class with the provided precision and scale
            Ok(decimal_cls
                .call1((precision, scale))?
                .into_pyobject(py)?
                .into())
        }
        PolarsDataType::Date => Ok(polars.getattr("Date")?.into_pyobject(py)?.into()),
        PolarsDataType::Datetime(time_unit, tz) => {
            // dbg!(&time_unit.to_string());
            // BUG: time_unit.to_string() returns the wrong value for the TimeUnit enum on microseconds
            //      causing a ValueError in python
            // bug code location: https://github.com/pola-rs/polars/blob/a578d51734657de85e2b5eb98b58065882f49d73/crates/polars-core/src/datatypes/time_unit.rs#L32
            // Example:
            // time_unit == TimeUnit::Microseconds
            // time_unit.to_string() => "μs"
            // => https://github.com/pola-rs/polars/blob/a578d51734657de85e2b5eb98b58065882f49d73/py-polars/polars/datatypes/classes.py#L531
            //   File ".venv/lib/python3.13/site-packages/polars/datatypes/classes.py", line 485, in __init__
            //     raise ValueError(msg)
            // ValueError: invalid `time_unit`
            //
            // Expected one of {'ns','us','ms'}, got 'μs'.
            //
            let tu = match time_unit {
                polars::datatypes::TimeUnit::Milliseconds => "ms",
                polars::datatypes::TimeUnit::Microseconds => "us",
                polars::datatypes::TimeUnit::Nanoseconds => "ns",
            };
            let datetime_cls = polars.getattr("Datetime")?;
            let pytz = tz.as_ref().map(|t| t.to_string());

            Ok(datetime_cls.call1((tu, pytz))?.into())
            // Ok(datetime_cls.call1((time_unit.to_string(), pytz))?.into_pyobject(py)?.into())
        }
        // add more data types here ...
        _ => Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
            format!("Conversion for DataType {:?} is not implemented", dtype),
        )),
    }
}

fn rust_to_pyarrow_dtype(py: Python, dt: &arrow::datatypes::DataType) -> PyResult<PyObject> {
    let pa = PyModule::import(py, "pyarrow")?;
    let dtype_obj = match dt {
        Int8 => pa.getattr("int8")?.call0(),
        Int16 => pa.getattr("int16")?.call0(),
        Int32 => pa.getattr("int32")?.call0(),
        Int64 => pa.getattr("int64")?.call0(),
        UInt8 => pa.getattr("uint8")?.call0(),
        UInt16 => pa.getattr("uint16")?.call0(),
        UInt32 => pa.getattr("uint32")?.call0(),
        UInt64 => pa.getattr("uint64")?.call0(),
        Float16 => pa.getattr("float16")?.call0(),
        Float32 => pa.getattr("float32")?.call0(),
        Float64 => pa.getattr("float64")?.call0(),
        Boolean => pa.getattr("bool_")?.call0(),
        Utf8 => pa.getattr("utf8")?.call0(),
        LargeUtf8 => pa.getattr("large_utf8")?.call0(),
        Binary => pa.getattr("binary")?.call0(),
        LargeBinary => pa.getattr("large_binary")?.call0(),
        FixedSizeBinary(size) => pa.getattr("fixed_size_binary")?.call1((size,)),
        Date32 => pa.getattr("date32")?.call0(),
        Date64 => pa.getattr("date64")?.call0(),
        Time32(unit) => {
            let unit_str = match unit {
                TimeUnit::Second => "s",
                TimeUnit::Millisecond => "ms",
                _ => {
                    return Err(PyValueError::new_err(
                        "Unsupported TimeUnit for Time32: expected Second or Millisecond",
                    ))
                }
            };
            pa.getattr("time32")?.call1((unit_str,))
        }
        Time64(unit) => {
            let unit_str = match unit {
                TimeUnit::Microsecond => "us",
                TimeUnit::Nanosecond => "ns",
                _ => {
                    return Err(PyValueError::new_err(
                        "Unsupported TimeUnit for Time64: expected Microsecond or Nanosecond",
                    ))
                }
            };
            pa.getattr("time64")?.call1((unit_str,))
        }
        Timestamp(unit, tz_opt) => {
            let unit_str = match unit {
                TimeUnit::Second => "ms",
                TimeUnit::Millisecond => "ms",
                TimeUnit::Microsecond => "us",
                TimeUnit::Nanosecond => "ns",
            };
            match tz_opt {
                Some(tz) => pa.getattr("timestamp")?.call1((unit_str, tz.to_string())),
                None => pa.getattr("timestamp")?.call1((unit_str,)),
            }
        }
        Interval(interval_unit) => match interval_unit {
            IntervalUnit::YearMonth => pa.getattr("month_interval")?.call0(),
            IntervalUnit::DayTime => pa.getattr("day_time_interval")?.call0(),
            IntervalUnit::MonthDayNano => pa.getattr("month_day_nano_interval")?.call0(),
        },
        Decimal128(precision, scale) => pa.getattr("decimal128")?.call1((precision, scale)),
        List(field) => {
            // for list types, we have to convert the inner field first
            let field_obj = field.to_pyarrow_field(py)?;
            pa.getattr("list_")?.call1((field_obj,))
        }
        LargeList(field) => {
            let field_obj = field.to_pyarrow_field(py)?;
            pa.getattr("large_list")?.call1((field_obj,))
        }
        Struct(fields) => {
            let py_fields = fields
                .iter()
                .map(|f| f.to_pyarrow_field(py))
                .collect::<PyResult<Vec<PyObject>>>()?;
            pa.getattr("struct")?.call1((py_fields,))
        }
        // more conversions ...
        _ => {
            return Err(PyNotImplementedError::new_err(format!(
                "Data type {:?} not implemented",
                dt
            )))
        }
    }?;
    Ok(dtype_obj.into())
}

fn convert_metadata(py: Python, metadata: &HashMap<String, String>) -> PyObject {
    if metadata.is_empty() {
        py.None()
    } else {
        let dict = PyDict::new(py);
        for (k, v) in metadata.iter() {
            // k and v are both &String; these automatically convert to Python strings
            dict.set_item(k, v).unwrap();
        }
        dict.into()
    }
}

/// Converts a Rust Arrow `Field` to a PyArrow field.
fn rust_to_pyarrow_field(py: Python, field: &Field) -> PyResult<PyObject> {
    let pa = PyModule::import(py, "pyarrow")?;
    let dtype = rust_to_pyarrow_dtype(py, field.data_type())?;

    let py_metadata = convert_metadata(py, field.metadata());

    let py_field =
        pa.getattr("field")?
            .call1((field.name(), dtype, field.is_nullable(), py_metadata))?;
    Ok(py_field.into())
}

pub trait PyArrowFieldConversion {
    fn to_pyarrow_field(&self, py: Python) -> PyResult<PyObject>;
}

impl PyArrowFieldConversion for Field {
    fn to_pyarrow_field(&self, py: Python) -> PyResult<PyObject> {
        rust_to_pyarrow_field(py, self)
    }
}

pub trait PyArrowSchemaConversion {
    fn to_pyarrow_schema(&self, py: Python) -> PyResult<PyObject>;
}

impl PyArrowSchemaConversion for Schema {
    fn to_pyarrow_schema(&self, py: Python) -> PyResult<PyObject> {
        let pa = PyModule::import(py, "pyarrow")?;
        let py_fields: Vec<PyObject> = self
            .fields()
            .iter()
            .map(|field| field.to_pyarrow_field(py))
            .collect::<PyResult<Vec<_>>>()?;

        let py_metadata = {
            let metadata = self.metadata();
            if metadata.is_empty() {
                py.None()
            } else {
                let dict = PyDict::new(py);
                for (k, v) in metadata.iter() {
                    dict.set_item(k, v)?;
                }
                dict.into()
            }
        };

        let kwargs = [("metadata", py_metadata)].into_py_dict(py)?;
        // passing a reference to kwargs
        let schema_obj = pa.getattr("schema")?.call((py_fields,), Some(&kwargs))?;
        Ok(schema_obj.into())
    }
}

#[pyfunction(signature = (xsd_file, format, timestamp_options=None, encoding="utf-8", lowercase=false))]
fn parse_xsd(
    py: Python,
    xsd_file: PathBuf,
    format: SchemaFormat,
    timestamp_options: Option<TimestampOptions>,
    encoding: &str,
    lowercase: Option<bool>,
) -> PyResult<PyObject> {
    let use_encoding = Encoding::for_label(encoding.as_bytes()).unwrap_or(UTF_8);
    let result = parse_file(xsd_file, timestamp_options, Some(use_encoding), lowercase);

    match result {
        Ok(schema) => {
            match format {
                SchemaFormat::Json => match schema.into_pyobject(py) {
                    Ok(py_schema) => Ok(py_schema.into()),
                    Err(e) => Err(e),
                },
                SchemaFormat::Avro => match schema.to_avro() {
                    Ok(avro_schema) => match avro_schema.into_pyobject(py) {
                        Ok(py_avro) => Ok(py_avro.into_pyobject(py)?.into()),
                        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            "Error converting to avro",
                        )),
                    },
                    Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "{}",
                        e
                    ))),
                },
                SchemaFormat::Arrow => match schema.to_arrow() {
                    Ok(arrow_schema) => match arrow_schema.to_pyarrow_schema(py) {
                        Ok(py_arrow) => Ok(py_arrow),
                        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            "Error converting to arrow",
                        )),
                    },
                    Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "{}",
                        e
                    ))),
                },
                SchemaFormat::Spark => match schema.to_spark() {
                    Ok(spark) => {
                        let pyspark_module = PyModule::import(py, "pyspark.sql.types")?;
                        let pyspark_struct_type = pyspark_module.getattr("StructType")?;
                        let dict_schema = spark.into_pyobject(py)?;
                        let py_struct_type = pyspark_struct_type
                            .getattr("fromJson")?
                            .call1((dict_schema,))?;
                        Ok(py_struct_type.into())
                    }
                    Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "{}",
                        e
                    ))),
                },
                SchemaFormat::JsonSchema => {
                    let json_schema = schema.to_json_schema();
                    let string_schema = json_schema.to_string();
                    let json_module = PyModule::import(py, "json")?;
                    let py_json_schema = json_module.getattr("loads")?.call1((string_schema,))?;
                    Ok(py_json_schema.into())
                }
                SchemaFormat::Duckdb => {
                    let duckdb_indexmap = schema.to_duckdb_schema();
                    let duckdb_schema = PyDict::new(py);
                    for (key, value) in duckdb_indexmap {
                        duckdb_schema.set_item(key, value)?;
                    }
                    match duckdb_schema.into_pyobject(py) {
                        Ok(py_duckdb_schema) => Ok(py_duckdb_schema.into()),
                        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            "Error converting to duckdb schema",
                        )),
                    }
                }
                SchemaFormat::Polars => {
                    let polars_schema = schema.to_polars();
                    let py_schema = PyDict::new(py);

                    for (name, dtype) in polars_schema.iter() {
                        let py_dtype = convert_polars_dtype_to_pyobject(py, dtype)?;
                        py_schema.set_item(name.to_string(), py_dtype)?;
                    }
                    Ok(py_schema.into_pyobject(py)?.into())
                } // _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                  //     format!("Invalid format: {}. Supported formats: arrow, duckdb, json, json-schema, polars, spark.", format))),
            }
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "{}",
            e
        ))),
    }
}

// main entrypoint for python module
#[pymodule]
fn pyaxp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_xsd, m)?)?;
    Ok(())
}
