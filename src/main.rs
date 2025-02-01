use yaxp::xsdp::parser::parse_file;

fn main() {
    let result = parse_file("example.xsd");

    match result {
        Ok(schema) => {
            schema.write_to_file("example-output.json").unwrap();
            let arrow_schema = schema.to_arrow().unwrap();
            dbg!(arrow_schema);
            println!("✅ Done!")
        }
        Err(e) => eprintln!("❌ {}", e),
    }

}
