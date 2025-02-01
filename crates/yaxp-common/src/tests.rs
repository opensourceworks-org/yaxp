#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::xsdp::parser::{parse_file, Schema, SchemaElement};

    #[test]
    fn test_parse_file() {
        let xsd_content = r#"
        <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:element name="example" type="xs:string"/>
        </xs:schema>
        "#;

        fs::write("test.xsd", xsd_content).unwrap();
        let result = parse_file("test.xsd");
        assert!(result.is_ok());
        let schema = result.unwrap();
        assert_eq!(schema.schema_element.name, "example");
        fs::remove_file("test.xsd").unwrap();
    }

    #[test]
    fn test_to_arrow() {
        let schema_element = SchemaElement {
            id: "example".to_string(),
            name: "example".to_string(),
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
            xpath: "/example".to_string(),
            nullable: Some(false),
            elements: vec![],
        };

        let schema = Schema::new(None, schema_element);
        let arrow_schema = schema.to_arrow().unwrap();
        assert_eq!(arrow_schema.fields().len(), 0);
    }

    #[test]
    fn test_write_to_file() {
        let schema_element = SchemaElement {
            id: "example".to_string(),
            name: "example".to_string(),
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
            xpath: "/example".to_string(),
            nullable: Some(false),
            elements: vec![],
        };

        let schema = Schema::new(None, schema_element);
        schema.write_to_file("test-output.json").unwrap();
        let output = fs::read_to_string("test-output.json").unwrap();
        assert!(output.contains("\"name\": \"example\""));
        fs::remove_file("test-output.json").unwrap();
    }
}