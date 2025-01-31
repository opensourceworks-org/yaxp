use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct XsdElement {
    id: String,
    name: String,
    dataType: Option<String>,
    minOccurs: Option<String>,
    maxOccurs: Option<String>,
    minLength: Option<String>,
    maxLength: Option<String>,
    pattern: Option<String>,
    fractionDigits: Option<String>,
    totalDigits: Option<String>,
    minInclusive: Option<String>,
    maxInclusive: Option<String>,
    values: Option<Vec<String>>,
    isCurrency: bool,
    xpath: String,
    elements: Vec<XsdElement>,
}

#[derive(Debug, Serialize, Deserialize)]
struct XsdSchema {
    namespace: Option<String>,
    schemaElement: XsdElement,
}

fn parse_xsd(file_path: &str) -> Result<XsdSchema, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut xml_reader = Reader::from_reader(reader);

    let mut buf = Vec::new();
    let mut schema_element: Option<XsdElement> = None;
    let mut current_element_stack: Vec<XsdElement> = Vec::new();
    let mut temp_element: Option<XsdElement> = None;

    while let Ok(event) = xml_reader.read_event_into(&mut buf) {
        match event {
            Event::Start(e) => {
                if e.name().as_ref() == b"xs:element" {
                    let name = get_attribute_value(&e, QName(b"name")).unwrap_or_default();
                    let data_type = get_attribute_value(&e, QName(b"type"));
                    let min_occurs = get_attribute_value(&e, QName(b"minOccurs")).or(Some("1".to_string()));
                    let max_occurs = get_attribute_value(&e, QName(b"maxOccurs"));

                    let xpath = if let Some(parent) = current_element_stack.last() {
                        format!("{}/{}", parent.xpath, name)
                    } else {
                        name.clone()
                    };

                    let new_element = XsdElement {
                        id: xpath.replace("/", "_"),
                        name: name.clone(),
                        dataType: data_type.clone(),
                        minOccurs: min_occurs.clone(),
                        maxOccurs: max_occurs.clone(),
                        minLength: None,
                        maxLength: None,
                        pattern: None,
                        fractionDigits: None,
                        totalDigits: None,
                        minInclusive: None,
                        maxInclusive: None,
                        values: None,
                        isCurrency: false,
                        xpath: xpath.clone(),
                        elements: Vec::new(),
                    };

                    temp_element = Some(new_element);
                }

                if let Some(ref mut temp) = temp_element {
                    match e.name().as_ref() {
                        b"xs:restriction" => {
                            temp.dataType = get_attribute_value(&e, QName(b"base"));
                        }
                        b"xs:maxLength" => {
                            temp.maxLength = get_attribute_value(&e, QName(b"value"));
                        }
                        b"xs:pattern" => {
                            temp.pattern = get_attribute_value(&e, QName(b"value"));
                        }
                        b"xs:enumeration" => {
                            let value = get_attribute_value(&e, QName(b"value"));
                            if let Some(v) = value {
                                temp.values.get_or_insert(Vec::new()).push(v);
                            }
                        }
                        b"xs:fractionDigits" => {
                            temp.fractionDigits = get_attribute_value(&e, QName(b"value"));
                        }
                        b"xs:totalDigits" => {
                            temp.totalDigits = get_attribute_value(&e, QName(b"value"));
                        }
                        _ => {}
                    }
                }

                if e.name().as_ref() == b"xs:complexType" || e.name().as_ref() == b"xs:sequence" {
                    if let Some(temp) = temp_element.take() {
                        current_element_stack.push(temp);
                    }
                }
            }
            Event::End(e) => {
                if e.name().as_ref() == b"xs:element" {
                    if let Some(finished_element) = temp_element.take() {
                        if let Some(parent) = current_element_stack.last_mut() {
                            parent.elements.push(finished_element);
                        } else {
                            schema_element = Some(finished_element);
                        }
                    }
                }

                if e.name().as_ref() == b"xs:complexType" || e.name().as_ref() == b"xs:sequence" {
                    let finished_element = current_element_stack.pop();
                    if let Some(finished) = finished_element {
                        if let Some(parent) = current_element_stack.last_mut() {
                            parent.elements.push(finished);
                        } else {
                            schema_element = Some(finished);
                        }
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }

        buf.clear();
    }

    Ok(XsdSchema {
        namespace: None,
        schemaElement: schema_element.unwrap(),
    })
}

fn get_attribute_value(e: &quick_xml::events::BytesStart, key: QName) -> Option<String> {
    e.attributes()
        .filter_map(|a| a.ok())
        .find(|a| a.key == key)
        .and_then(|a| String::from_utf8(a.value.into_owned()).ok())
}

fn main() {
    match parse_xsd("example.xsd") {
        Ok(schema) => {
            let json_output = serde_json::to_string_pretty(&schema).unwrap();
            println!("{}", json_output);
        }
        Err(e) => eprintln!("Error parsing XSD: {}", e),
    }
}
