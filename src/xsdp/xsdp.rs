use roxmltree::Document;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
// use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Schema {
    namespace: Option<String>,
    #[serde(rename = "schemaElement")]
    schema_element: SchemaElement,
}

#[derive(Serialize, Deserialize, Debug)]
struct SchemaElement {
    id: String,
    name: String,
    #[serde(rename = "dataType")]
    data_type: Option<String>,
    #[serde(rename = "minOccurs")]
    min_occurs: Option<String>,
    #[serde(rename = "maxOccurs")]
    max_occurs: Option<String>,
    #[serde(rename = "minLength")]
    min_length: Option<String>,
    #[serde(rename = "maxLength")]
    max_length: Option<String>,
    #[serde(rename = "minExclusive")]
    min_exclusive: Option<String>,
    #[serde(rename = "maxExclusive")]
    max_exclusive: Option<String>,
    #[serde(rename = "minInclusive")]
    min_inclusive: Option<String>,
    #[serde(rename = "maxInclusive")]
    max_inclusive: Option<String>,
    pattern: Option<String>,
    #[serde(rename = "fractionDigits")]
    fraction_digits: Option<String>,
    #[serde(rename = "totalDigits")]
    total_digits: Option<String>,
    values: Option<Vec<String>>,
    #[serde(rename = "isCurrency")]
    is_currency: bool,
    xpath: String,
    elements: Vec<SchemaElement>,
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
}

// Extract values from xs:enumeration
fn extract_enum_values(node: roxmltree::Node) -> Option<Vec<String>> {
    let mut values = Vec::new();
    for child in node.children() {
        if child.tag_name().name() == "enumeration" {
            if let Some(value) = child.attribute("value") {
                values.push(value.to_string());
            }
        }
    }
    if values.is_empty() { None } else { Some(values) }
}

// Extract constraints from xs:restriction
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
    };

    for child in node.children() {
        match child.tag_name().name() {
            "minLength" => simple_type.min_length = child.attribute("value").map(String::from),
            "maxLength" => simple_type.max_length = child.attribute("value").map(String::from),
            "minInclusive" => simple_type.min_inclusive = child.attribute("value").map(String::from),
            "maxInclusive" => simple_type.max_inclusive = child.attribute("value").map(String::from),
            "minExclusive" => simple_type.min_exclusive = child.attribute("value").map(String::from),
            "maxExclusive" => simple_type.max_exclusive = child.attribute("value").map(String::from),
            "fractionDigits" => simple_type.fraction_digits = child.attribute("value").map(String::from),
            "totalDigits" => simple_type.total_digits = child.attribute("value").map(String::from),
            "pattern" => simple_type.pattern = child.attribute("value").map(String::from),
            _ => {}
        }
    }
    simple_type
}

// Parse an xs:element, applying global type constraints
fn parse_element(node: roxmltree::Node, parent_xpath: &str, global_types: &HashMap<String, SimpleType>) -> Option<SchemaElement> {
    if node.tag_name().name() != "element" {
        return None;
    }

    let name = node.attribute("name")?.to_string();
    let xpath = format!("{}/{}", parent_xpath, name);
    let mut data_type = node.attribute("type").map(|s| s.replace("xs:", ""));
    let min_occurs = match node.attribute("minOccurs") {
        None => Some("1".to_string()),
        Some(m) => Some(m.to_string())
    };

    let max_occurs = match node.attribute("maxOccurs"){
        Some(m) => Some(m.to_string()),
        None => Some("1".to_string())
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
        elements,
    })
}

pub fn parse_file(xsd_file: &str, output_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let xml_content = fs::read_to_string(xsd_file).expect("Failed to read XSD file");
    let doc = Document::parse(&xml_content).expect("Failed to parse XML");

    let mut global_types = HashMap::new();

    for node in doc.root().descendants() {
        if node.tag_name().name() == "simpleType" {
            if let Some(name) = node.attribute("name") {
                for child in node.children() {
                    if child.tag_name().name() == "restriction" {
                        global_types.insert(name.to_string(), extract_constraints(child));
                    }
                }
            }
        }
    }

    let mut schema_element = None;

    for node in doc.root().descendants() {

        if node.tag_name().name() == "element" {
            schema_element = parse_element(node, &node.attribute("name").unwrap(), &global_types);
            break;
        }
    }

    if let Some(schema_element) = schema_element {
        let schema = Schema {
            namespace: None,
            schema_element,
        };

        let json_output = serde_json::to_string_pretty(&schema).expect("Failed to serialize JSON");

        fs::write(output_file, json_output).expect("Failed to write JSON");

        // println!("✅ JSON output saved to 'example-output.json'");
        Ok(format!("JSON output saved to '{}'", output_file))
    } else {
        // eprintln!("❌ Failed to find the main schema element in the XSD.");
        Err("Failed to find the main schema element in the XSD.".into())
    }


}
