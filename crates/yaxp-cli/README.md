<p align="center">
  <a href="https://crates.io/crates/yaxp-cli">
    <img alt="version" src="https://img.shields.io/crates/v/yaxp-cli">
  </a>
  <a href="https://crates.io/crates/yaxp-cli">
    <img alt="downloads" src="https://img.shields.io/crates/d/yaxp-cli">
  </a>
  <a href="https://github.com/opensourceworks-org/yaxp/blob/main/crates/yaxp-cli/README.md">
    <img alt="pipelines" src="https://img.shields.io/github/actions/workflow/status/opensourceworks-org/yaxp/yaxp-cli-ci.yml?logo=github">
  </a>

</p>

# **<yaxp ⚡> Yet Another XSD Parser**


## Introduction
Using [roxmltree](https://github.com/RazrFalcon/roxmltree) to parse XML files. 

Converts xsd schema to:
- [x] arrow
- [ ] avro
- [x] duckdb (read_csv columns/types)
- [x] json
- [x] json representation of spark schema
- [x] jsonschema
- [ ] polars
- [ ] protobuf


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
