from pyaxp import parse_xsd

schema = parse_xsd("example.xsd", "json")

def test_parse_schema():
    assert isinstance(schema, dict)
    name = schema["schema_element"]["id"]
    element_count = len(schema["schema_element"]["elements"])
    assert name == "Main_Element"