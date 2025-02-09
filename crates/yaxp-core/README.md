<p align="center">
  <a href="https://crates.io/crates/yaxp-core">
    <img alt="versions" src="https://img.shields.io/crates/v/yaxp-core">
  </a>
  <a href="https://crates.io/crates/yaxp-core">
    <img alt="downloads" src="https://img.shields.io/crates/d/yaxp-core">
  </a>
  <a href="https://github.com/opensourceworks-org/yaxp/blob/main/crates/yaxp-core/README.md">
    <img alt="pipelines" src="https://img.shields.io/github/actions/workflow/status/opensourceworks-org/yaxp/yaxp-core.yml?logo=github">
  </a>
</p>

# **<yaxp ⚡> Yet Another XSD Parser**

> 📌 **Note:** This project is still under heavy development, and its APIs are subject to change.


## Introduction
Using [roxmltree](https://github.com/RazrFalcon/roxmltree) to parse XML files. 

Converts xsd schema to:
- [x] arrow
- [x] avro
- [x] duckdb (read_csv columns/types)
- [x] json
- [x] json representation of spark schema
- [x] jsonschema
- [x] polars
- [ ] protobuf



## TODO

- [x] Add pyo3/maturin support (crate pyaxp)
- [ ] Add more tests
