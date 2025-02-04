<p align="center">
  <a href="https://crates.io/crates/yaxp-common">
    <img alt="versions" src="https://img.shields.io/crates/v/yaxp-common?label=yaxp-common">
  </a>
  <a href="https://crates.io/crates/yaxp-common">
    <img alt="downloads" src="https://img.shields.io/crates/d/yaxp-common?label=yaxp-common downloads">
  </a>
  <a href="https://crates.io/crates/yaxp-cli">
    <img alt="versions" src="https://img.shields.io/crates/v/yaxp-cli?label=yaxp-cli">
  </a>
  <a href="https://crates.io/crates/yaxp-common">
    <img alt="downloads" src="https://img.shields.io/crates/d/yaxp-cli?label=yaxp-cli downloads">
  </a>
  <a href="https://pypi.org/project/pyaxp/">
    <img alt="version" src="https://img.shields.io/pypi/v/pyaxp.svg?label=pyaxp">
  </a>  
  <a href="https://pypi.org/project/pyaxp/">
    <img alt="downloads" src="https://img.shields.io/pypi/dm/pyaxp?label=pyaxp downloads">
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
- [x] polars
- [ ] protobuf


## Project Structure

```
crates
├── pyaxp               -> Python bindings 
├── yaxp-cli            -> cli version
└── yaxp-common         -> lib
```

## Downloads

- [yaxp-common](https://crates.io/crates/yaxp-common) main lib crate, core
- [yaxp-cli](https://crates.io/crates/yaxp-cli) cli interface on yaxp-common
- [pyaxp](https://pypi.org/project/pyaxp/) python bindings




## TODO

- [x] Add pyo3/maturin support
- [ ] add parameter for timezone unit/TZ (testing with polars)
- [ ] Add tests
