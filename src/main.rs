use yaxp::xsdp::xsdp;

fn main() {
    let result = xsdp::parse_file("example.xsd", "example-output.json");

    match result {
        Ok(msg) => println!("✅ {}. Done!", msg),
        Err(e) => eprintln!("❌ {}", e),
    }

}