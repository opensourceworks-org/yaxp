use arrow::datatypes::{DataType, Field, Schema as ArrowSchema, TimeUnit};
use pyo3::IntoPyObject;
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use serde_json::json;
use rayon::iter::{ParallelBridge, ParallelIterator};
use polars::datatypes::{DataType as PolarsDataType, PlSmallStr};
use polars::datatypes::TimeUnit as PolarsTimeUnit;
use polars::prelude::{Schema as PolarsSchema};

#[derive(Serialize, Deserialize, Debug, IntoPyObject)]
pub struct Schema {
    pub(crate) namespace: Option<String>,
    #[serde(rename = "schemaElement")]
    pub schema_element: SchemaElement,
}

impl Schema {
    pub fn new(namespace: Option<String>, schema_element: SchemaElement) -> Self {
        Schema {
            namespace,
            schema_element,
        }
    }

    pub fn to_arrow(&self) -> Result<ArrowSchema, Box<dyn std::error::Error>> {
        let mut fields = vec![];

        for element in &self.schema_element.elements {
            let field = Field::new(
                &element.name,
                element.to_arrow()?,
                element.nullable.unwrap_or(true),
            )
            .with_metadata(element.to_metadata());
            fields.push(field);
        }

        Ok( ArrowSchema::new(fields))
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
        json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "Main_Element": self.schema_element.to_json_schema()
            },
            "required": ["Main_Element"]
        })
    }

    pub fn to_duckdb_schema(&self) -> IndexMap<String, String> {
        self.schema_element.to_duckdb_schema()
    }

    pub fn to_polars(&self) -> PolarsSchema {
        let mut schema: PolarsSchema = Default::default();
        for element in &self.schema_element.elements {
            //let field = polars::datatypes::Field::new(PlSmallStr::from(&element.name), element.to_polars());
            schema.insert(PlSmallStr::from(&element.name), element.to_polars());
        };

        schema


        // let fields = self.schema_element.elements.iter().map(|element| {
        //     polars::datatypes::Field::new(PlSmallStr::from(&element.name), element.to_polars())
        // }).collect();
        // polars::prelude::Schema::new(fields)
        // let fields_vec = if !self.schema_element.elements.is_empty() {
        //     self
        //         .schema_element
        //         .elements
        //         .iter()
        //         .map(|e| {polars::datatypes::Field::new(PlSmallStr::from(&e.name), e.to_polars())})
        //         .collect()
        // } else {
        //     Vec::new()
        // };
        //
        //
        // // Construct the Polars schema using the IndexMap.
        // let polars_schema: PolarsSchema = fields_vec.into();
        // polars_schema


    }


}

#[derive(Serialize, Deserialize, Debug, IntoPyObject)]
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

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let json_output = serde_json::to_string(&self).expect("Failed to serialize JSON");
        Ok(json_output)
    }
}

#[derive(Serialize, Deserialize, Debug, IntoPyObject, Clone)]
pub struct SparkField {
    #[serde(rename = "name")]
    pub field_name: String,
    #[serde(rename = "type")]
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

#[derive(Serialize, Deserialize, Debug, IntoPyObject, Clone)]
pub struct SchemaElement {
    pub id: String,
    pub name: String,
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
                "decimal" => {
                    match (&self.total_digits, &self.fraction_digits) {
                        (Some(precision), Some(scale)) => Ok(DataType::Decimal128(
                            precision.parse::<u8>().unwrap(),
                            scale.parse::<i8>().unwrap(),
                        )),
                        _ => Ok(DataType::Float64),
                    }
                }
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
                if let (Some(ref total_digits), Some(ref fraction_digits)) = (
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
            Some(other) => other.to_string(), // todo: pass through the provided type?
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

    fn to_json_schema(&self) -> serde_json::Value {
        let mut properties = serde_json::Map::new();
        let mut required = vec![];

        for element in &self.elements {
            let mut field_type = serde_json::Map::new();
            let base_type = match element.data_type.as_deref() {
                Some("string") => json!("string"),
                Some("integer") => json!("integer"),
                Some("decimal") => json!("number"),
                Some("date") => json!("string"),
                Some("dateTime") => json!("string"),
                _ => json!("string"),
            };

            let final_type = if element.nullable == Some(true) {
                json!([base_type, "null"])
            } else {
                base_type
            };

            field_type.insert("type".to_string(), final_type);

            if let Some(max_length) = &element.max_length {
                field_type.insert("maxLength".to_string(), json!(max_length.parse::<u64>().unwrap_or(255)));
            }
            if let Some(min_length) = &element.min_length {
                field_type.insert("minLength".to_string(), json!(min_length.parse::<u64>().unwrap_or(0)));
            }
            if let Some(pattern) = &element.pattern {
                field_type.insert("pattern".to_string(), json!(pattern));
            }
            if let Some(values) = &element.values {
                field_type.insert("enum".to_string(), json!(values));
            }
            if element.data_type.as_deref() == Some("decimal") {
                if let (Some(fraction_digits), Some(total_digits)) = (
                    element.fraction_digits.as_deref(),
                    element.total_digits.as_deref(),
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

            properties.insert(element.name.clone(), serde_json::Value::Object(field_type));
            if element.nullable == Some(false) {
                required.push(element.name.clone());
            }
        }

        json!({
            "type": "object",
            "properties": properties,
            "required": required,
        })
    }

    fn to_duckdb_schema(&self) -> IndexMap<String, String> {
        let mut columns = IndexMap::new();

        for element in &self.elements {
            let column_type = match element.data_type.as_deref() {
                Some("string") => format!("VARCHAR({})", element.max_length.as_deref().unwrap_or("255")),
                Some("integer") => "INTEGER".to_string(),
                Some("decimal") => {
                    let precision = element.total_digits.as_deref().unwrap_or("25");
                    let scale = element.fraction_digits.as_deref().unwrap_or("7");
                    format!("DECIMAL({}, {})", precision, scale)
                },
                Some("date") => "DATE".to_string(),
                Some("dateTime") => "TIMESTAMP".to_string(),
                _ => "VARCHAR(255)".to_string(),
            };

            columns.insert(element.name.clone(), column_type);
        }

        columns
    }

    fn to_polars(&self) -> PolarsDataType {
        match self.data_type.as_deref() {
            None => PolarsDataType::String,
            Some("string") => PolarsDataType::String,
            Some("int") | Some("integer") => PolarsDataType::Int64,
            Some("float") | Some("double") => PolarsDataType::Float64,
            Some("boolean") | Some("bool") => PolarsDataType::Boolean,
            Some("date") => PolarsDataType::Date,
            Some("datetime") | Some("dateTime") => PolarsDataType::Datetime(PolarsTimeUnit::Milliseconds, None),
            Some("time") => PolarsDataType::Time,
            Some("decimal") => {
                // Parse the total_digits as precision and fraction_digits as scale.
                // Fallback to defaults if parsing fails.
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

#[derive(Debug)]
struct SimpleType {
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

// from xs:restriction
fn extract_constraints(node: roxmltree::Node) -> SimpleType {
    let mut simple_type = SimpleType {
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
    global_types: &HashMap<String, SimpleType>,
) -> Option<SchemaElement> {
    if node.tag_name().name() != "element" {
        return None;
    }

    let name = node.attribute("name")?.to_string();
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
                for subchild in child.descendants() {
                    if let Some(sub_element) = parse_element(subchild, &xpath, global_types) {
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
    })
}

pub fn parse_file(xsd_file: &str) -> Result<Schema, Box<dyn std::error::Error>> {
    let xml_content = fs::read_to_string(xsd_file).expect("Failed to read XSD file");
    let doc = Document::parse(&xml_content).expect("Failed to parse XML");

    let global_types = Arc::new(Mutex::new(HashMap::new()));

    doc.root().descendants()
        .par_bridge()
        .for_each(|node| {
            if node.tag_name().name() == "simpleType" {
                if let Some(name) = node.attribute("name") {
                    for child in node.children() {
                        if child.tag_name().name() == "restriction" {
                            let mut map = global_types.lock().unwrap();
                            map.insert(name.to_string(), extract_constraints(child));
                            // global_types.insert(name.clone().to_string(), extract_constraints(child));
                        }
                    }
                }
            }
        });
        // if node.tag_name().name() == "simpleType" {
        //     if let Some(name) = node.attribute("name") {
        //         for child in node.children() {
        //             if child.tag_name().name() == "restriction" {
        //                 global_types.insert(name.to_string(), extract_constraints(child));
        //             }
        //         }
        //     }
        // }

    let final_map = Arc::try_unwrap(global_types)
        .expect("Arc should have no other refs")
        .into_inner()
        .expect("Mutex should be unlocked");


    // for node in doc.root().descendants() {
    //     if node.tag_name().name() == "simpleType" {
    //         if let Some(name) = node.attribute("name") {
    //             for child in node.children() {
    //                 if child.tag_name().name() == "restriction" {
    //                     global_types.insert(name.to_string(), extract_constraints(child));
    //                 }
    //             }
    //         }
    //     }
    // }

    let mut schema_element = None;

    for node in doc.root().descendants() {
        if node.tag_name().name() == "element" {
            schema_element = parse_element(node, node.attribute("name").unwrap(), &final_map);
            break;
        }
    }

    if let Some(schema_element) = schema_element {
        let schema = Schema {
            namespace: None,
            schema_element,
        };

        Ok(schema)
    } else {
        Err("Failed to find the main schema element in the XSD.".into())
    }
}
