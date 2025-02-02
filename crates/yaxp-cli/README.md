<p align="center">
  <a href="https://crates.io/crates/yaxp-cli">
    <img alt="version" src="https://img.shields.io/crates/v/yaxp-cli">
  </a>
  <a href="https://crates.io/crates/yaxp-cli">
    <img alt="downloads" src="https://img.shields.io/crates/d/yaxp-cli">
  </a>
</p>

# **<yaxp ⚡> Yet Another XSD Parser**


## Introduction
Using [roxmltree](https://github.com/RazrFalcon/roxmltree) to parse XML files. 

Converts xsd schema to:
- [x] json
- [x] arrow
- [ ] avro
- [ ] protobuf
- [x] jsonschema
- [x] json representation of spark schema
- [x] duckdb (read_csv columns/types)


## Usage

```shell
$ ./yaxp-cli --help
<yaxp-cli ⚡> Yet Another XSD Parser

Usage: yaxp-cli [OPTIONS] --xsd <XSD>

Options:
  -x, --xsd <XSD>        Path to the XSD file
  -f, --format <FORMAT>  Output format: json (default), arrow [default: json] [possible values: json, arrow]
  -o, --output <OUTPUT>  optional output filename
  -h, --help             Print help
  -V, --version          Print version
$
 ```


## TODO

- [x] Add pyo3/maturin support (crate pyaxp)
- [ ] Add tests
