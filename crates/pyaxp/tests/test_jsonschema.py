from pyaxp import parse_xsd

schema = parse_xsd("example.xsd", "json-schema")

def test_schema_type():
    assert isinstance(schema, dict)
def test_parse_schema():
    assert "Main_Element" in schema["properties"].keys()
    assert schema["required"] == ["Field1", "Field2", "Field3", "Field5", "Field8", "Field15", "Field17"]