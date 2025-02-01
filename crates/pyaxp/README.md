# **<yaxp ⚡>**
Yet Another XSD Parser

## Introduction
Using [roxmltree](https://github.com/RazrFalcon/roxmltree) to parse XML files. 

Converts xsd schema to:
- [x] json
- [x] arrow
- [ ] avro
- [ ] protobuf
- [ ] jsonschema
- [ ] json representation of spark schema
- [ ] duckdb

## User Guide
### Python
- create and activate a Python virtual environment (or use poetry, uv, etc.)
- install maturin (cargo install, pip install into venv, etc.)

```shell
(.venv)  ~/projects/yaxp/crates/pyaxp $
🔗 Found pyo3 bindings
🐍 Found CPython 3.12 at ~/projects/yaxp/crates/pyaxp/.venv/bin/python
📡 Using build options features from pyproject.toml
warning: ~/projects/yaxp/Cargo.toml: unused manifest key: workspace.name
    Blocking waiting for file lock on build directory
   Compiling pyo3-build-config v0.23.4
   Compiling pyo3-macros-backend v0.23.4
   Compiling pyo3-ffi v0.23.4
   Compiling pyo3 v0.23.4
   Compiling pyo3-macros v0.23.4
   Compiling yaxp-common v0.1.0 (~/Users/jeroen~/projects/yaxp/crates/yaxp-common)
   Compiling pyaxp v0.1.0 (~/Users/jeroen~/projects/yaxp/crates/pyaxp)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.03s
📦 Built wheel for CPython 3.12 to /var/folders/gr/gl3fzn_n0_g4fzpcfv2g40gh0000gn/T/.tmp3wQ0CY/pyaxp-0.1.0-cp312-cp312-macosx_11_0_arm64.whl
✏️  Setting installed package as editable
🛠 Installed pyaxp-0.1.0
(.venv)  ~/projects/yaxp/crates/pyaxp $
```

```python
Python 3.12.3 (main, Apr 15 2024, 17:43:11) [Clang 17.0.6 ] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyaxp import parse_xsd
>>> import json
>>> j = parse_xsd("example.xsd")
>>> type(j)
<class 'dict'>
>>> print(json.dumps(j, indent=4))
{
    "namespace": null,
    "schema_element": {
        "id": "Main_Element",
        "name": "Main_Element",
        "data_type": null,
        "min_occurs": "1",
        "max_occurs": "1",
        "min_length": null,
        "max_length": null,
        "min_exclusive": null,
        "max_exclusive": null,
        "min_inclusive": null,
        "max_inclusive": null,
        "pattern": null,
        "fraction_digits": null,
        "total_digits": null,
        "values": null,
        "is_currency": false,
        "xpath": "Main_Element/Main_Element",
        "nullable": null,
        "elements": [
            {
                "id": "Field1",
                "name": "Field1",
                "data_type": "string",
                "min_occurs": "1",
                "max_occurs": "1",
                "min_length": null,
                "max_length": "15",
                "min_exclusive": null,
                "max_exclusive": null,
                "min_inclusive": null,
                "max_inclusive": null,
                "pattern": null,
                "fraction_digits": null,
                "total_digits": null,
                "values": null,
                "is_currency": false,
                "xpath": "Main_Element/Main_Element/Field1",
                "nullable": false,
                "elements": []
            },
            {
                "id": "Field2",
                "name": "Field2",
                "data_type": "string",
                "min_occurs": "1",
                "max_occurs": "1",
                "min_length": null,
                "max_length": "20",
                "min_exclusive": null,
                "max_exclusive": null,
                "min_inclusive": null,
                "max_inclusive": null,
                "pattern": null,
                "fraction_digits": null,
                "total_digits": null,
                "values": null,
                "is_currency": false,
                "xpath": "Main_Element/Main_Element/Field2",
                "nullable": false,
                "elements": []
            },
            {
                "id": "Field3",
                "name": "Field3",
                "data_type": "string",
                "min_occurs": "1",
                "max_occurs": "1",
                "min_length": null,
                "max_length": "10",
                "min_exclusive": null,
                "max_exclusive": null,
                "min_inclusive": null,
                "max_inclusive": null,
                "pattern": null,
                "fraction_digits": null,
                "total_digits": null,
                "values": null,
                "is_currency": false,
                "xpath": "Main_Element/Main_Element/Field3",
                "nullable": false,
                "elements": []
            }, ...
```


## TODO

- [x] Add pyo3/maturin support
- [ ] Add tests
