use roxmltree::Document;
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Write;

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
    pattern: Option<String>,
    #[serde(rename = "fractionDigits")]
    fraction_digits: Option<String>,
    #[serde(rename = "totalDigits")]
    total_digits: Option<String>,
    #[serde(rename = "minInclusive")]
    min_inclusive: Option<String>,
    #[serde(rename = "maxInclusive")]
    max_inclusive: Option<String>,
    #[serde(rename = "minExclusive")]
    min_exclusive: Option<String>,
    #[serde(rename = "maxExclusive")]
    max_exclusive: Option<String>,
    values: Option<Vec<String>>,
    #[serde(rename = "isCurrency")]
    is_currency: bool,
    xpath: String,
    elements: Vec<SchemaElement>,
}

// Extract enumeration values from xs:restriction
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
fn extract_constraints(node: roxmltree::Node) -> (Option<String>, Option<String>, Option<String>, Option<String>, Option<String>) {
    let mut max_length = None;
    let mut fraction_digits = None;
    let mut total_digits = None;
    let mut min_inclusive = None;
    let mut min_exclusive = None;

    for child in node.children() {
        match child.tag_name().name() {
            "maxLength" => max_length = child.attribute("value").map(String::from),
            "fractionDigits" => fraction_digits = child.attribute("value").map(String::from),
            "totalDigits" => total_digits = child.attribute("value").map(String::from),
            "minInclusive" => min_inclusive = child.attribute("value").map(String::from),
            "minExclusive" => min_exclusive = child.attribute("value").map(String::from),
            _ => {}
        }
    }
    (max_length, fraction_digits, total_digits, min_inclusive, min_exclusive)
}

// Parse an xs:element recursively
fn parse_element(node: roxmltree::Node, parent_xpath: &str) -> Option<SchemaElement> {
    if node.tag_name().name() != "element" {
        return None;
    }

    let name = node.attribute("name")?.to_string();
    let xpath = format!("{}/{}", parent_xpath, name);
    let mut data_type = node.attribute("type").map(String::from);
    let min_occurs = node.attribute("minOccurs").map(String::from);
    let max_occurs = node.attribute("maxOccurs").map(String::from);
    
    let mut max_length = None;
    let mut fraction_digits = None;
    let mut total_digits = None;
    let mut min_inclusive = None;
    let mut min_exclusive = None;
    let pattern = None;
    let mut values = None;

    let mut elements = Vec::new();
    
    for child in node.children() {
        match child.tag_name().name() {
            "simpleType" => {
                for subchild in child.children() {
                    if subchild.tag_name().name() == "restriction" {
                        if let Some(base) = subchild.attribute("base") {
                            data_type = Some(base.replace("xs:", ""));
                        }
                        values = extract_enum_values(subchild);
                        let constraints = extract_constraints(subchild);
                        max_length = constraints.0;
                        fraction_digits = constraints.1;
                        total_digits = constraints.2;
                        min_inclusive = constraints.3;
                    }
                }
            }
            "complexType" => {
                for subchild in child.descendants() {
                    if let Some(sub_element) = parse_element(subchild, &xpath) {
                        elements.push(sub_element);
                    }
                }
            }
            _ => {}
        }
    }

    let is_currency = name == "Currency";

    Some(SchemaElement {
        id: format!("{}_{}", parent_xpath, name),
        name,
        data_type,
        min_occurs,
        max_occurs,
        min_length: None,
        max_length,
        pattern,
        fraction_digits,
        total_digits,
        min_inclusive,
        max_inclusive: None,
        min_exclusive,
        max_exclusive: None,
        values,
        is_currency,
        xpath,
        elements,
    })
}

fn main() {
    let xml_content = fs::read_to_string("example.xsd").expect("Failed to read XSD file");
    let doc = Document::parse(&xml_content).expect("Failed to parse XML");

    let mut schema_element = None;

    for node in doc.root().descendants() {
        if node.tag_name().name() == "element" && node.attribute("name") == Some("Main_Element") {
            schema_element = parse_element(node, "Main_Element");
            break;
        }
    }

    if let Some(schema_element) = schema_element {
        let schema = Schema {
            namespace: None,
            schema_element,
        };

        let json_output = serde_json::to_string_pretty(&schema).expect("Failed to serialize JSON");

        let mut file = fs::File::create("example-output.json").expect("Unable to create output file");
        file.write_all(json_output.as_bytes()).expect("Failed to write JSON");

        println!("✅ JSON output saved to 'example-output.json'");
    } else {
        eprintln!("❌ Failed to find the main schema element in the XSD.");
    }
}
