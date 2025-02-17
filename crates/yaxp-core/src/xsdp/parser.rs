use arrow::datatypes::{DataType, Field, Schema as ArrowSchema, TimeUnit};
use encoding_rs::{Encoding, UTF_8};
use encoding_rs_io::DecodeReaderBytesBuilder;
use indexmap::IndexMap;
use polars::datatypes::TimeUnit as PolarsTimeUnit;
use polars::datatypes::{DataType as PolarsDataType, PlSmallStr};
use polars::prelude::Schema as PolarsSchema;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::{PyAnyMethods, PyDictMethods};
#[cfg(feature = "python")]
use pyo3::types::{PyDict, PyString};
#[cfg(feature = "python")]
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyResult, Python};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
#[cfg(feature = "python")]
use std::convert::Infallible;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::{fmt, fs};
use std::path::PathBuf;

/// Converting the `TimestampUnit` enum to a string representation for Polars breaks
/// on "μs" when passing from rust to python. We handle that here by converting it to "us".

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TimestampUnit {
    Ms,
    Us,
    Ns,
}

impl FromStr for TimestampUnit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ns" => Ok(TimestampUnit::Ns),
            "ms" => Ok(TimestampUnit::Ms),
            "us" => Ok(TimestampUnit::Us),
            "μs" => Ok(TimestampUnit::Us),
            _ => Err(format!("Invalid precision: {}. Available: ms, us, ns", s)),
        }
    }
}

#[cfg(feature = "python")]
impl<'py> IntoPyObject<'py> for TimestampUnit {
    type Target = <&'py str as IntoPyObject<'py>>::Target;
    type Output = <&'py str as IntoPyObject<'py>>::Output;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<pyo3::Bound<'py, PyString>, Infallible> {
        let s = match self {
            TimestampUnit::Ms => "ms",
            TimestampUnit::Us => "us",
            TimestampUnit::Ns => "ns",
        };
        s.into_pyobject(py)
    }
}

impl fmt::Display for TimestampUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
pub struct TimestampOptions {
    pub time_unit: Option<TimestampUnit>,
    pub time_zone: Option<String>,
}

#[cfg(feature = "python")]
impl<'source> FromPyObject<'source> for TimestampUnit {
    fn extract_bound(bound: &pyo3::Bound<'source, PyAny>) -> PyResult<Self> {
        let s: String = <String as FromPyObject>::extract_bound(bound)?;

        // clippy advises to replace the closure with the function itself: `PyValueError::new_err`
        // TimestampUnit::from_str(&s).map_err(|e| PyValueError::new_err(e))
        TimestampUnit::from_str(&s).map_err(PyValueError::new_err)
    }
}

#[cfg(feature = "python")]
fn _get_extracted_string(dict: &Bound<PyDict>, key: &str) -> PyResult<Option<String>> {
    if let Some(item) = dict.get_item(key)? {
        Ok(Some(item.extract()?))
    } else {
        Ok(None)
    }
}

#[cfg(feature = "python")]
impl<'source> FromPyObject<'source> for TimestampOptions {
    fn extract_bound(bound: &pyo3::Bound<'source, PyAny>) -> PyResult<Self> {
        let obj = bound;
        let dict = obj.downcast::<PyDict>()?;

        let time_unit: Option<String> = _get_extracted_string(dict, "time_unit")?;
        let time_zone: Option<String> = _get_extracted_string(dict, "time_zone")?;

        let time_unit = match time_unit {
            Some(s) => Some(s.parse().map_err(|e: String| PyValueError::new_err(e))?),
            None => None,
        };

        Ok(TimestampOptions {
            time_unit,
            time_zone,
        })
    }
}

fn map_avro_data_type(dt: &str) -> AvroType {
    match dt.to_lowercase().as_str() {
        // primitive types
        "null" | "xs:null" => AvroType::Simple("null".to_string()),
        "boolean" | "xs:boolean" => AvroType::Simple("boolean".to_string()),
        "int" | "xs:int" => AvroType::Simple("int".to_string()),
        "long" | "xs:long" => AvroType::Simple("long".to_string()),
        "float" | "xs:float" => AvroType::Simple("float".to_string()),
        "double" | "xs:double" => AvroType::Simple("double".to_string()),
        "bytes" | "xs:bytes" | "xs:base64binary" => AvroType::Simple("bytes".to_string()),
        "string" | "xs:string" => AvroType::Simple("string".to_string()),

        // logical types (these are typically built on top of primitive types)
        "date" | "xs:date" => AvroType::Logical { base: "int".to_string(), logical: "date".to_string() },
        "time-millis" | "xs:time" => AvroType::Logical { base: "int".to_string(), logical: "time-millis".to_string() },
        "timestamp-millis" | "xs:datetime" => AvroType::Logical { base: "long".to_string(), logical: "timestamp-millis".to_string() },
        "timestamp-micros" => AvroType::Logical { base: "long".to_string(), logical: "timestamp-micros".to_string() },

        // complex types (placeholders – constructing full schemas for these requires extra info)
        "array" => AvroType::Simple("array".to_string()),
        "map" => AvroType::Simple("map".to_string()),
        "record" => AvroType::Simple("record".to_string()),
        "enum" => AvroType::Simple("enum".to_string()),
        "fixed" => AvroType::Simple("fixed".to_string()),

        // default to "string"
        _ => AvroType::Simple("string".to_string()),
    }
}




/// Parsing the XSD file and converting it to various formats begins with a Schema struct
/// and a SchemaElement struct.  Currently supporting Arrow, Spark, JSON, JSON Schema,
/// DuckDB, and Polars schemas.
///

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
pub struct Schema {
    pub namespace: Option<String>,
    #[serde(rename = "schemaElement")]
    pub schema_element: SchemaElement,
    pub timestamp_options: Option<TimestampOptions>,
    pub doc: Option<String>,
    pub custom_types: Option<IndexMap<String, SimpleType>>,
}

impl Schema {
    pub fn new(
        namespace: Option<String>,
        schema_element: SchemaElement,
        timestamp_options: Option<TimestampOptions>,
        doc: Option<String>,
        custom_types: Option<IndexMap<String, SimpleType>>,
    ) -> Self {
        Schema {
            namespace,
            schema_element,
            timestamp_options,
            doc,
            custom_types,
        }
    }

    pub fn to_avro(&self) -> Result<AvroSchema, Box<dyn std::error::Error>> {
        let schema = AvroSchema {
            schema_type: "record".to_string(),
            name: self.schema_element.name.clone(),
            namespace: self.namespace.clone(),
            aliases: None, // Or derive aliases if available.
            fields: self.schema_element.to_avro_fields(),
            doc: None,
        };
        Ok(schema)
    }

    pub fn to_arrow(&self) -> Result<ArrowSchema, Box<dyn std::error::Error>> {
        let fields = self
            .schema_element
            .elements
            .par_iter()
            .map(|element| {
                Field::new(
                    &element.name,
                    element.to_arrow().unwrap(),
                    element.nullable.unwrap_or(true),
                )
                .with_metadata(element.to_metadata())
            })
            .collect::<Vec<Field>>();

        Ok(ArrowSchema::new(fields))
    }

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let json_output = serde_json::to_string(&self).expect("Failed to serialize JSON");
        Ok(json_output)
    }

    pub fn write_to_json_file(&self, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_output = serde_json::to_string_pretty(&self).expect("Failed to serialize JSON");
        fs::write(output_file, json_output).expect("Failed to write JSON");
        Ok(())
    }

    pub fn to_spark(&self) -> Result<SparkSchema, Box<dyn std::error::Error>> {
        let mut fields = vec![];

        for element in &self.schema_element.elements {
            fields.push(element.to_spark()?);
        }

        let schema = SparkSchema::new("struct".to_string(), fields);

        Ok(schema)
    }

    pub fn to_json_schema(&self) -> serde_json::Value {
        let mut fields = vec![];
        let mut required = vec![];

        for element in &self.schema_element.elements {
            let (field, nullable) = element.to_json_schema();
            fields.push(field);
            if !nullable {
                required.push(element.name.clone());
            }
        }

        json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                format!("{}", &self.schema_element.name): {
                    "type": "object",
                    "properties": fields.iter().map(|field| {
                        let obj = field.as_object().unwrap();
                        let (key, value) = obj.iter().next().unwrap(); // Assumes each field has one key
                        json!({ "key": key, "value": value })
                    }).collect::<Vec<_>>(),
                }
            },
            "required": required
        })
    }

    pub fn to_duckdb_schema(&self) -> IndexMap<String, String> {
        // self.schema_element.to_duckdb_schema()
        let mut columns = IndexMap::new();
        for element in &self.schema_element.elements {
            let mut element_columns = element.to_duckdb_schema();
            columns.append(&mut element_columns);
        }
        columns
    }

    pub fn to_polars(&self) -> PolarsSchema {
        let mut schema: PolarsSchema = Default::default();
        let to = self.timestamp_options.clone();

        for element in &self.schema_element.elements {
            //let field = polars::datatypes::Field::new(PlSmallStr::from(&element.name), element.to_polars());
            schema.insert(PlSmallStr::from(&element.name), element.to_polars(&to));
        }
        schema
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
pub struct SchemaElement {
    pub id: String,
    pub name: String,
    pub documentation: Option<String>,
    #[serde(rename = "dataType")]
    pub data_type: Option<String>,
    #[serde(rename = "minOccurs")]
    pub min_occurs: Option<String>,
    #[serde(rename = "maxOccurs")]
    pub max_occurs: Option<String>,
    #[serde(rename = "minLength")]
    pub min_length: Option<String>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<String>,
    #[serde(rename = "minExclusive")]
    pub min_exclusive: Option<String>,
    #[serde(rename = "maxExclusive")]
    pub max_exclusive: Option<String>,
    #[serde(rename = "minInclusive")]
    pub min_inclusive: Option<String>,
    #[serde(rename = "maxInclusive")]
    pub max_inclusive: Option<String>,
    pub pattern: Option<String>,
    #[serde(rename = "fractionDigits")]
    pub fraction_digits: Option<String>,
    #[serde(rename = "totalDigits")]
    pub total_digits: Option<String>,
    pub values: Option<Vec<String>>,
    #[serde(rename = "isCurrency")]
    pub is_currency: bool,
    pub xpath: String,
    pub nullable: Option<bool>,
    pub elements: Vec<SchemaElement>,
}

impl SchemaElement {
    pub fn to_metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();

        if let Some(ref max_occurs) = self.max_occurs {
            metadata.insert("maxOccurs".to_string(), max_occurs.clone());
        }
        if let Some(ref min_length) = self.min_length {
            metadata.insert("minLength".to_string(), min_length.clone());
        }
        if let Some(ref max_length) = self.max_length {
            metadata.insert("maxLength".to_string(), max_length.clone());
        }
        if let Some(ref min_exclusive) = self.min_exclusive {
            metadata.insert("minExclusive".to_string(), min_exclusive.clone());
        }
        if let Some(ref max_exclusive) = self.max_exclusive {
            metadata.insert("maxExclusive".to_string(), max_exclusive.clone());
        }
        if let Some(ref min_inclusive) = self.min_inclusive {
            metadata.insert("minInclusive".to_string(), min_inclusive.clone());
        }
        if let Some(ref max_inclusive) = self.max_inclusive {
            metadata.insert("maxInclusive".to_string(), max_inclusive.clone());
        }
        if let Some(ref pattern) = self.pattern {
            metadata.insert("pattern".to_string(), pattern.clone());
        }
        if let Some(ref values) = self.values {
            // have to join the vector of values into a single comma-separated string
            metadata.insert("values".to_string(), values.join(","));
        }
        // may want to add explicitly, check with xsd specification
        if self.is_currency {
            metadata.insert("isCurrency".to_string(), self.is_currency.to_string());
        }

        metadata
    }

    pub fn to_arrow(&self) -> Result<DataType, Box<dyn std::error::Error>> {
        if let Some(ref data_type) = self.data_type {
            match data_type.as_str() {
                "string" => Ok(DataType::Utf8),
                "integer" => Ok(DataType::Int32),
                "decimal" => match (&self.total_digits, &self.fraction_digits) {
                    (Some(precision), Some(scale)) => Ok(DataType::Decimal128(
                        precision.parse::<u8>().unwrap(),
                        scale.parse::<i8>().unwrap(),
                    )),
                    _ => Ok(DataType::Float64),
                },
                "boolean" => Ok(DataType::Boolean),
                "date" => Ok(DataType::Date32),
                "dateTime" => Ok(DataType::Timestamp(TimeUnit::Nanosecond, None)),

                _ => Ok(DataType::Utf8),
            }
        } else {
            Ok(DataType::Utf8)
        }
    }

    pub fn to_spark(&self) -> Result<SparkField, Box<dyn std::error::Error>> {
        let field_type = match &self.data_type.as_deref() {
            Some("decimal") => {
                if let (Some(total_digits), Some(fraction_digits)) = (
                    &self.total_digits.as_deref(),
                    &self.fraction_digits.as_deref(),
                ) {
                    let precision = total_digits.parse::<u32>().unwrap_or(0);
                    let scale = fraction_digits.parse::<u32>().unwrap_or(0);
                    format!("decimal({}, {})", precision, scale)
                } else {
                    "decimal".to_string()
                }
            }
            Some("int") | Some("integer") => "integer".to_string(),
            Some("long") => "long".to_string(),
            Some("float") => "float".to_string(),
            Some("double") => "double".to_string(),
            Some("boolean") => "boolean".to_string(),
            Some("dateTime") => "timestamp".to_string(),
            Some("date") => "date".to_string(),
            Some("string") => "string".to_string(),
            Some(other) => other.to_string(), // todo: should we really just pass through the provided type?
            None => "string".to_string(),
        };

        let field = SparkField {
            field_name: self.name.clone(),
            field_type,
            nullable: self.nullable.unwrap_or(true),
            metadata: Some(self.to_metadata()),
        };

        Ok(field)
    }

    fn to_json_schema(&self) -> (serde_json::Value, bool) {
        let mut field_type = serde_json::Map::new();
        let base_type = match self.data_type.as_deref() {
            Some("string") => json!("string"),
            Some("integer") => json!("integer"),
            Some("decimal") => json!("number"),
            Some("date") => json!("string"),
            Some("dateTime") => json!("string"),
            _ => json!("string"),
        };

        let final_type = if self.nullable == Some(true) {
            json!([base_type, "null"])
        } else {
            base_type
        };

        field_type.insert("type".to_string(), final_type);

        if let Some(max_length) = &self.max_length {
            field_type.insert(
                "maxLength".to_string(),
                json!(max_length.parse::<u64>().unwrap_or(255)),
            );
        }
        if let Some(min_length) = &self.min_length {
            field_type.insert(
                "minLength".to_string(),
                json!(min_length.parse::<u64>().unwrap_or(0)),
            );
        }
        if let Some(pattern) = &self.pattern {
            field_type.insert("pattern".to_string(), json!(pattern));
        }
        if let Some(values) = &self.values {
            field_type.insert("enum".to_string(), json!(values));
        }
        if self.data_type.as_deref() == Some("decimal") {
            if let (Some(fraction_digits), Some(total_digits)) = (
                self.fraction_digits.as_deref(),
                self.total_digits.as_deref(),
            ) {
                let fraction = fraction_digits.parse::<u64>().unwrap_or(0);
                let total = total_digits.parse::<u64>().unwrap_or(0);
                let multiple_of = 10f64.powi(-(fraction as i32));
                let max_value = 10f64.powi(total as i32) - multiple_of;

                field_type.insert("multipleOf".to_string(), json!(multiple_of));
                field_type.insert("minimum".to_string(), json!(0));
                field_type.insert("maximum".to_string(), json!(max_value));
            }
        }

        (
            json!({
                &self.name: field_type

            }),
            self.nullable.unwrap_or(true),
        )
    }

    pub fn to_avro_field(&self) -> AvroField {
        let base_type = self.to_avro_type();
        let field_type = if self.nullable.unwrap_or(false) {
            AvroType::Union(vec![AvroType::Simple("null".to_string()), base_type])
        } else {
            base_type
        };

        AvroField {
            name: self.name.clone(),
            field_type,
            doc: self.documentation.clone(),
        }
    }

    pub fn to_avro_type(&self) -> AvroType {
        if !self.elements.is_empty() {
            let fields = self
                .elements
                .iter()
                .map(|child| child.to_avro_field())
                .collect();
            let record = AvroSchema {
                schema_type: "record".to_string(),
                name: self.name.clone(),
                namespace: None,
                aliases: None,
                doc: self.documentation.clone(),
                fields,
            };
            AvroType::Record(record)
        } else if let Some(symbols) = &self.values {
            let avro_enum = AvroEnum {
                schema_type: "enum".to_string(),
                name: self.name.clone(),
                symbols: symbols.clone(),
                doc: self.documentation.clone(),
                namespace: None,
            };
            AvroType::Enum(avro_enum)
        } else if let Some(dt) = &self.data_type {
            map_avro_data_type(dt)
        } else {
            // default type.
            AvroType::Simple("string".to_string())
        }
    }

    pub fn to_avro_fields(&self) -> Vec<AvroField> {
        self.elements
            .iter()
            .map(|child| child.to_avro_field())
            .collect()
    }

    fn to_duckdb_schema(&self) -> IndexMap<String, String> {
        let mut columns = IndexMap::new();

        let column_type = match self.data_type.as_deref() {
            Some("string") => format!("VARCHAR({})", self.max_length.as_deref().unwrap_or("255")),
            Some("integer") => "INTEGER".to_string(),
            Some("decimal") => {
                let precision = self.total_digits.as_deref().unwrap_or("25");
                let scale = self.fraction_digits.as_deref().unwrap_or("7");
                format!("DECIMAL({}, {})", precision, scale)
            }
            Some("date") => "DATE".to_string(),
            Some("dateTime") => "TIMESTAMP".to_string(),
            _ => "VARCHAR(255)".to_string(),
        };

        columns.insert(self.name.clone(), column_type);

        columns
    }

    fn to_polars(&self, timestamp_options: &Option<TimestampOptions>) -> PolarsDataType {
        match self.data_type.as_deref() {
            None => PolarsDataType::String,
            Some("string") => PolarsDataType::String,
            Some("int") | Some("integer") => PolarsDataType::Int64,
            Some("float") | Some("double") => PolarsDataType::Float64,
            Some("boolean") | Some("bool") => PolarsDataType::Boolean,
            Some("date") => PolarsDataType::Date,
            Some("datetime") | Some("dateTime") => {
                let time_unit = timestamp_options
                    .as_ref()
                    .and_then(|options| options.time_unit.as_ref())
                    .map(|unit| match unit {
                        TimestampUnit::Ms => PolarsTimeUnit::Milliseconds,
                        TimestampUnit::Us => PolarsTimeUnit::Microseconds,
                        TimestampUnit::Ns => PolarsTimeUnit::Nanoseconds,
                    })
                    .unwrap_or(PolarsTimeUnit::Nanoseconds);
                let timezone = timestamp_options
                    .as_ref()
                    .and_then(|options| options.time_zone.as_ref())
                    .map(|s| s.into());
                PolarsDataType::Datetime(time_unit, timezone)
            }
            Some("time") => PolarsDataType::Time,
            Some("decimal") => {
                // parsing the total_digits as precision and fraction_digits as scale
                // fallback to defaults if parsing fails: 38|10
                let precision = self
                    .total_digits
                    .as_ref()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(38);
                let scale = self
                    .fraction_digits
                    .as_ref()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(10);
                PolarsDataType::Decimal(Some(precision), Some(scale))
            }
            Some(other) => {
                eprintln!(
                    "Warning: Unrecognized data type '{}', defaulting to String.",
                    other
                );
                PolarsDataType::String
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
pub struct AvroSchema {
    #[serde(rename = "type")]
    #[cfg_attr(feature = "python", pyo3(item("type")))]
    pub schema_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    pub fields: Vec<AvroField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
pub struct AvroField {
    pub name: String,
    #[serde(rename = "type")]
    #[cfg_attr(feature = "python", pyo3(item("type")))]
    pub field_type: AvroType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
}

// #[derive(Serialize, Deserialize, Debug, IntoPyObject)]
// pub struct AvroLogical {
//
// }
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[serde(untagged)]
pub enum AvroType {
    /// simple type (like "string", "int", etc.)
    Simple(String),
    /// union of types (ie. a nullable field).
    Union(Vec<AvroType>),
    /// inline record.
    Record(AvroSchema),
    Enum(AvroEnum),
    Logical {
        #[serde(rename = "type")]
        #[cfg_attr(feature = "python", pyo3(item("type")))]
        base: String,
        #[serde(rename = "logicalType")]
        #[cfg_attr(feature = "python", pyo3(item("logicalType")))]
        logical: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
pub struct AvroEnum {
    #[serde(rename = "type")]
    #[cfg_attr(feature = "python", pyo3(item("type")))]
    pub schema_type: String, // must be "enum"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub name: String,
    pub symbols: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
pub struct SparkSchema {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub fields: Vec<SparkField>,
}

impl SparkSchema {
    pub fn new(schema_type: String, fields: Vec<SparkField>) -> Self {
        SparkSchema {
            schema_type,
            fields,
        }
    }

    pub fn to_json(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let json_output = serde_json::to_value(self).expect("Failed to serialize JSON");
        Ok(json_output)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
pub struct SparkField {
    #[serde(rename = "name")]
    #[cfg_attr(feature = "python", pyo3(item("name")))]
    pub field_name: String,
    #[serde(rename = "type")]
    #[cfg_attr(feature = "python", pyo3(item("type")))]
    pub field_type: String,
    pub nullable: bool,
    pub metadata: Option<HashMap<String, String>>,
}

impl SparkField {
    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let json_output = serde_json::to_string(&self).expect("Failed to serialize JSON");
        Ok(json_output)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
pub struct SimpleType {
    name: Option<String>,
    data_type: Option<String>,
    min_length: Option<String>,
    max_length: Option<String>,
    min_inclusive: Option<String>,
    max_inclusive: Option<String>,
    min_exclusive: Option<String>,
    max_exclusive: Option<String>,
    fraction_digits: Option<String>,
    total_digits: Option<String>,
    pattern: Option<String>,
    values: Option<Vec<String>>,
    nullable: Option<bool>,
    documentation: Option<String>,
}


// xs:enumeration
fn extract_enum_values(node: roxmltree::Node) -> Option<Vec<String>> {
    let mut values = Vec::new();
    for child in node.children() {
        if child.tag_name().name() == "enumeration" {
            if let Some(value) = child.attribute("value") {
                values.push(value.to_string());
            }
        }
    }
    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

// xs:annotation
fn extract_documentation(node: roxmltree::Node) -> Option<String> {
    for child in node.children() {
        if child.tag_name().name() == "documentation" {
            //dbg!(&child.text().map(String::from));
            return child.text().map(String::from);
        }
    }

    None
}

// from xs:restriction
fn extract_constraints(node: roxmltree::Node) -> SimpleType {
    let mut simple_type = SimpleType {
        name: None,
        data_type: node.attribute("base").map(|s| s.replace("xs:", "")),
        min_length: None,
        max_length: None,
        min_inclusive: None,
        max_inclusive: None,
        min_exclusive: None,
        max_exclusive: None,
        fraction_digits: None,
        total_digits: None,
        pattern: None,
        values: extract_enum_values(node),
        nullable: None,
        documentation: extract_documentation(node),
    };

    for child in node.children() {
        match child.tag_name().name() {
            "minLength" => simple_type.min_length = child.attribute("value").map(String::from),
            "maxLength" => simple_type.max_length = child.attribute("value").map(String::from),
            "minInclusive" => {
                simple_type.min_inclusive = child.attribute("value").map(String::from)
            }
            "maxInclusive" => {
                simple_type.max_inclusive = child.attribute("value").map(String::from)
            }
            "minExclusive" => {
                simple_type.min_exclusive = child.attribute("value").map(String::from)
            }
            "maxExclusive" => {
                simple_type.max_exclusive = child.attribute("value").map(String::from)
            }
            "fractionDigits" => {
                simple_type.fraction_digits = child.attribute("value").map(String::from)
            }
            "totalDigits" => simple_type.total_digits = child.attribute("value").map(String::from),
            "pattern" => simple_type.pattern = child.attribute("value").map(String::from),
            "nullable" => simple_type.nullable = Some(true),
            _ => {}
        }
    }
    simple_type
}

fn parse_element(
    node: roxmltree::Node,
    parent_xpath: &str,
    global_types: &IndexMap<String, SimpleType>,
    lowercase: Option<bool>,
) -> Option<SchemaElement> {
    if node.tag_name().name() != "element" {
        return None;
    }

    let mut name = node.attribute("name")?.to_string();
    if lowercase.is_some() && lowercase.unwrap() {
        name = name.to_lowercase();
    }

    let nullable = node.attribute("nillable").map(|s| s == "true");
    let xpath = format!("{}/{}", parent_xpath, name);
    let mut data_type = node.attribute("type").map(|s| s.replace("xs:", ""));
    let min_occurs = match node.attribute("minOccurs") {
        None => Some("1".to_string()),
        Some(m) => Some(m.to_string()),
    };

    let max_occurs = match node.attribute("maxOccurs") {
        Some(m) => Some(m.to_string()),
        None => Some("1".to_string()),
    };

    let mut documentation = None;

    let mut min_length = None;
    let mut max_length = None;
    let mut min_inclusive = None;
    let mut max_inclusive = None;
    let mut min_exclusive = None;
    let mut max_exclusive = None;
    let mut fraction_digits = None;
    let mut total_digits = None;
    let mut pattern = None;
    let mut values = None;
    let mut elements = Vec::new();

    if let Some(ref type_name) = data_type {
        if let Some(global_type) = global_types.get(type_name) {
            min_length = global_type.min_length.clone();
            max_length = global_type.max_length.clone();
            min_inclusive = global_type.min_inclusive.clone();
            max_inclusive = global_type.max_inclusive.clone();
            min_exclusive = global_type.min_exclusive.clone();
            max_exclusive = global_type.max_exclusive.clone();
            fraction_digits = global_type.fraction_digits.clone();
            total_digits = global_type.total_digits.clone();
            pattern = global_type.pattern.clone();
            values = global_type.values.clone();
            data_type = global_type.data_type.clone();
            documentation = global_type.documentation.clone();
        }
    }

    for child in node.children() {
        match child.tag_name().name() {
            "simpleType" => {
                for subchild in child.children() {
                    if subchild.tag_name().name() == "restriction" {
                        let simple_type = extract_constraints(subchild);
                        if simple_type.data_type.is_some() {
                            data_type = simple_type.data_type;
                        }
                        min_length = simple_type.min_length;
                        max_length = simple_type.max_length;
                        min_inclusive = simple_type.min_inclusive;
                        max_inclusive = simple_type.max_inclusive;
                        min_exclusive = simple_type.min_exclusive;
                        max_exclusive = simple_type.max_exclusive;
                        fraction_digits = simple_type.fraction_digits;
                        total_digits = simple_type.total_digits;
                        pattern = simple_type.pattern;
                        values = simple_type.values;
                    }
                }
            }
            "complexType" => {
                //let q = extract_documentation(child);

                for subchild in child.descendants() {
                    if let Some(sub_element) = parse_element(subchild, &xpath, global_types, lowercase) {
                        elements.push(sub_element);
                    }
                }
            }
            _ => {}
        }
    }

    let is_currency = name == "Currency";

    Some(SchemaElement {
        id: name.clone(),
        name,
        data_type,
        min_occurs,
        max_occurs,
        min_length,
        max_length,
        min_inclusive,
        max_inclusive,
        min_exclusive,
        max_exclusive,
        pattern,
        fraction_digits,
        total_digits,
        values,
        is_currency,
        xpath,
        nullable,
        elements,
        documentation,
    })
}

pub fn read_xsd_file(xsd_file: PathBuf, encoding: Option<&'static Encoding>) -> Result<String, Box<dyn std::error::Error>> {
    let parsed_file = File::open(xsd_file);
    //.expect("Failed to read XSD file");

    if let Err(e) = parsed_file {
        return Err(format!("Failed to read XSD file: {}", e).into());
    }

    let file = parsed_file.unwrap();

    let use_encoding = encoding.unwrap_or(UTF_8);

    let mut transcode_reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(use_encoding))
        .build(file);

    let mut xml_content = String::new();
    transcode_reader.read_to_string(&mut xml_content)?;

    Ok(xml_content)
}

pub fn parse_xsd_string(xsd_string: &str, timestamp_options: Option<TimestampOptions>, lowercase: Option<bool>) -> Result<Schema, Box<dyn std::error::Error>> {
    let parse_doc = Document::parse(xsd_string);

    if let Err(e) = parse_doc {
        return Err(format!("Failed to parse XML: {}. Maybe try a different encoding (utf-16 ?).", e).into());
    }

    let doc = parse_doc.unwrap();
    let mut schema_doc: Option<String> = None;


    let global_types = Arc::new(Mutex::new(IndexMap::new()));

    doc.root().descendants().for_each(|node| {
        if node.tag_name().name() == "simpleType" {
            if let Some(name) = node.attribute("name") {

                let mut doc = None;
                for child in node.children() {
                    if child.tag_name().name() == "annotation" {
                        //println!("----------> {:?}", &child);
                        let d = extract_documentation(child);
                        if let Some(o) = d {
                            doc = Some(o.clone());
                        }
                    }
                    if child.tag_name().name() == "restriction" {
                        let mut map = global_types.lock().unwrap();
                        let mut st = extract_constraints(child);
                        st.name = Some(name.to_string());
                        st.documentation = doc.clone();
                        map.insert(name.to_string(), st);
                    }
                }
            }
        } else if node.tag_name().name() == "annotation" {
            if schema_doc.is_none() {
                schema_doc = extract_documentation(node);
            }
        }
    });

    let final_map = Arc::try_unwrap(global_types)
        .expect("Arc should have no other refs")
        .into_inner()
        .expect("Mutex should be unlocked");

    let mut schema_element = None;

    for node in doc.root().descendants() {
        if node.tag_name().name() == "element" {
            let mut element_name = "".to_string();
            if let Some(name) = node.attribute("name"){
                if lowercase.is_some() && lowercase.unwrap() {
                    element_name = name.to_lowercase();
                } else {
                    element_name = name.to_string();
                }
            }
            schema_element = parse_element(node, &element_name, &final_map, lowercase);
            break;
        }
    }

    let mut custom_types_vec: Vec<_> = final_map.into_iter().collect();
    custom_types_vec.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    let final_map: IndexMap<_, _> = custom_types_vec.into_iter().collect();

    if let Some(schema_element) = schema_element {
        let schema = Schema {
            namespace: None,
            schema_element,
            timestamp_options,
            doc: schema_doc,
            custom_types: Some(final_map),
        };

        Ok(schema)
    } else {
        Err("Failed to find the main schema element in the XSD.".into())
    }
}

pub fn parse_file(xsd_file: PathBuf, timestamp_options: Option<TimestampOptions>,
                  encoding: Option<&'static Encoding>,
                  lowercase: Option<bool>) -> Result<Schema, Box<dyn std::error::Error>> {
    let xml_content = read_xsd_file(xsd_file, encoding)?;

    parse_xsd_string(&xml_content, timestamp_options, lowercase)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_test_schema() -> Schema {
        let element1 = SchemaElement {
            id: "id".to_string(),
            name: "field1".to_string(),
            data_type: Some("string".to_string()),
            min_occurs: Some("1".to_string()),
            max_occurs: Some("1".to_string()),
            min_length: None,
            max_length: None,
            min_inclusive: None,
            max_inclusive: None,
            min_exclusive: None,
            max_exclusive: None,
            pattern: None,
            fraction_digits: None,
            total_digits: None,
            values: None,
            is_currency: false,
            xpath: "/name".to_string(),
            nullable: Some(false),
            elements: vec![],
            documentation: Some("This is the first test field".to_string()),
        };

        let element2 = SchemaElement {
            id: "id".to_string(),
            name: "field2".to_string(),
            data_type: Some("string".to_string()),
            min_occurs: Some("1".to_string()),
            max_occurs: Some("1".to_string()),
            min_length: None,
            max_length: None,
            min_inclusive: None,
            max_inclusive: None,
            min_exclusive: None,
            max_exclusive: None,
            pattern: None,
            fraction_digits: None,
            total_digits: None,
            values: None,
            is_currency: false,
            xpath: "/name".to_string(),
            nullable: Some(true),
            elements: vec![],
            documentation: Some("This is the second test field".to_string()),
        };
        let element3 = SchemaElement {
            id: "id".to_string(),
            name: "field3".to_string(),
            data_type: Some("string".to_string()),
            min_occurs: Some("1".to_string()),
            max_occurs: Some("1".to_string()),
            min_length: None,
            max_length: None,
            min_inclusive: None,
            max_inclusive: None,
            min_exclusive: None,
            max_exclusive: None,
            pattern: None,
            fraction_digits: None,
            total_digits: None,
            values: None,
            is_currency: false,
            xpath: "/name".to_string(),
            nullable: Some(true),
            elements: vec![],
            documentation: Some("This is the third and last test field".to_string()),
        };

        let schema = Schema {
            namespace: None,
            schema_element: SchemaElement {
                id: "id".to_string(),
                name: "main_schema".to_string(),
                data_type: Some("string".to_string()),
                min_occurs: Some("1".to_string()),
                max_occurs: Some("1".to_string()),
                min_length: None,
                max_length: None,
                min_inclusive: None,
                max_inclusive: None,
                min_exclusive: None,
                max_exclusive: None,
                pattern: None,
                fraction_digits: None,
                total_digits: None,
                values: None,
                is_currency: false,
                xpath: "/name".to_string(),
                nullable: Some(true),
                elements: vec![element1, element2, element3],
                documentation: Some("This is the main schema".to_string()),
            },
            timestamp_options: None,
            doc: Some("TestSchema".to_string()),
            custom_types: None,
        };

        schema
    }

    #[test]
    fn test_timestamp_unit_from_str() {
        assert_eq!(TimestampUnit::from_str("ns").unwrap(), TimestampUnit::Ns);
        assert_eq!(TimestampUnit::from_str("ms").unwrap(), TimestampUnit::Ms);
        assert_eq!(TimestampUnit::from_str("us").unwrap(), TimestampUnit::Us);
        assert!(TimestampUnit::from_str("invalid").is_err());
    }

    #[test]
    fn test_schema_to_arrow() {
        let schema = create_test_schema();
        let arrow_schema = schema.to_arrow().unwrap();
        assert_eq!(arrow_schema.fields().len(), 3);
        assert_eq!(arrow_schema.field(0).name(), "field1");
        assert_eq!(schema.doc, Some("TestSchema".to_string()));
    }

    #[test]
    fn test_parse_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.xsd");
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            r#"
            <schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
                <xs:element name="testElement" type="xs:string"/>
            </schema>
            "#
        )
        .unwrap();

        let schema = parse_file(file_path, None, None, Some(false)).unwrap();
        assert_eq!(schema.schema_element.name, "testElement");
    }

    #[test]
    fn test_parse_file_lowercase() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.xsd");
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            r#"
            <schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
                <xs:element name="testElement" type="xs:string"/>
            </schema>
            "#
        )
            .unwrap();

        let schema = parse_file(file_path, None, None, Some(true)).unwrap();
        dbg!(&schema);
        assert_eq!(schema.schema_element.name, "testelement");
    }
    #[test]
    fn test_schema_element_to_arrow() {
        let schema = create_test_schema();
        let element = &schema.schema_element.elements[0];

        let data_type = element.to_arrow().unwrap();
        assert_eq!(data_type, DataType::Utf8);
    }

    #[test]
    fn test_schema_element_to_spark() {
        let schema = create_test_schema();
        let element = &schema.schema_element.elements[0];

        let spark_field = element.to_spark().unwrap();
        assert_eq!(spark_field.field_name, "field1");
        assert_eq!(spark_field.field_type, "string");
    }

    #[test]
    fn test_schema_element_to_json_schema() {
        let schema = create_test_schema();
        let element = &schema.schema_element.elements[0];

        let (json_element, nullable) = element.to_json_schema();
        assert_eq!(
            json_element
                .get("field1")
                .and_then(|v| v.get("type"))
                .and_then(|v| v.as_str()),
            Some("string")
        );
        assert!(!nullable);
    }

    #[test]
    fn test_schema_element_to_duckdb_schema() {
        let schema = create_test_schema();
        let element = &schema.schema_element.elements[0];
        let duckdb_schema = element.to_duckdb_schema();
        assert_eq!(
            duckdb_schema.get("field1").unwrap().to_string(),
            "VARCHAR(255)"
        );
    }

    #[test]
    fn test_duckdb_schema_ordered() {
        let schema = create_test_schema();

        let duckdb_schema = schema.to_duckdb_schema();
        dbg!(&duckdb_schema);
        let names = duckdb_schema
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            &[
                "field1".to_string(),
                "field2".to_string(),
                "field3".to_string()
            ]
        );
    }

    // #[test]
    // fn test_schema_to_avro(){
    //     let schema = create_test_schema();
    //     let avro_schema = schema.to_avro().unwrap();
    //     assert_eq!(avro_schema.)
    // }

    #[test]
    fn test_extract_enum_values() {
        let xml = r#"
            <restriction base="xs:string">
                <enumeration value="A"/>
                <enumeration value="B"/>
            </restriction>
        "#;
        let doc = Document::parse(xml).unwrap();
        let node = doc.root().first_child().unwrap();
        let values = extract_enum_values(node).unwrap();
        assert_eq!(values, vec!["A", "B"]);
    }

    #[test]
    fn test_extract_constraints() {
        let xml = r#"
            <restriction base="xs:string">
                <minLength value="1"/>
                <maxLength value="255"/>
            </restriction>
        "#;
        let doc = Document::parse(xml).unwrap();
        let node = doc.root().first_child().unwrap();
        let constraints = extract_constraints(node);
        assert_eq!(constraints.min_length, Some("1".to_string()));
        assert_eq!(constraints.max_length, Some("255".to_string()));
    }

    #[test]
    fn test_parse_element() {
        let xml = r#"
            <element name="testElement" type="xs:string"/>
        "#;
        let doc = Document::parse(xml).unwrap();
        let node = doc.root().first_child().unwrap();
        let element = parse_element(node, "", &IndexMap::new(), Some(false)).unwrap();
        assert_eq!(element.name, "testElement");
        assert_eq!(element.data_type, Some("string".to_string()));
    }

    #[test]
    fn test_extract_documentation() {
        let xml = r#"
            <annotation>
                <documentation>This is a test element</documentation>
            </annotation>
        "#;
        let doc = Document::parse(xml).unwrap();
        let node = doc.root().first_child().unwrap();
        let documentation = extract_documentation(node);
        assert_eq!(documentation, Some("This is a test element".to_string()));
    }

    #[test]
    fn test_avro_schema_serialization() {
        let schema = AvroSchema {
            schema_type: "record".to_string(),
            namespace: Some("example.avro".to_string()),
            name: "LongList".to_string(),
            doc: Some("Linked list of 64-bit longs.".to_string()),
            aliases: Some(vec!["LinkedLongs".to_string()]),
            fields: vec![
                AvroField {
                    name: "value".to_string(),
                    field_type: AvroType::Simple("long".to_string()),
                    doc: Some("The value of the node.".to_string()),
                },
                AvroField {
                    name: "next".to_string(),
                    field_type: AvroType::Union(vec![
                        AvroType::Simple("null".to_string()),
                        AvroType::Simple("LongList".to_string()),
                    ]),
                    doc: Some("The next node in the list.".to_string()),
                },
            ],
        };
        assert_eq!(schema.fields.len(), 2);
        assert_eq!(schema.doc, Some("Linked list of 64-bit longs.".to_string()));
    }
}
