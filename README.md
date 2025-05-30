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
  <a href="https://crates.io/crates/yaxp-cli">
    <img alt="downloads" src="https://img.shields.io/crates/d/yaxp-cli?label=yaxp-cli downloads">
  </a>
  <a href="https://crates.io/crates/yaxp-core">
    <img alt="downloads" src="https://img.shields.io/crates/d/yaxp-core?label=yaxp-core downloads">
  </a>
  <a href="https://crates.io/crates/yaxp-core">
    <img alt="versions" src="https://img.shields.io/crates/v/yaxp-core?label=yaxp-core">
  </a>
  <a href="https://pypi.org/project/pyaxp/">
    <img alt="version" src="https://img.shields.io/pypi/v/pyaxp.svg?label=pyaxp">
  </a>  
  <a href="https://pypi.org/project/pyaxp/">
    <img alt="downloads" src="https://img.shields.io/pypi/dm/pyaxp?label=pyaxp downloads">
  </a>
</p>


# **<yaxp ⚡> Yet Another XSD Parser**

> 📌 **Note:** This project is still under heavy development, and its APIs are subject to change.

> **🏃 RUNNING EXAMPLE USING WASM**    
>[<xsd ⚡> convert](https://xsd-convert.com)



## Introduction
Using [roxmltree](https://github.com/RazrFalcon/roxmltree) to parse XML files. 

Couldn't find a good xsd parser that can convert xsd schema to popular data processing library formats.  
This project will read custom SimpleType and ComplexType from xsd schema and convert them to other formats, trying to stay as close to the original schema as possible.
Even though arrow might be common to most, each library has its own quirks and limitations.

Converts xsd schema to:
- [x] arrow
- [x] avro
- [x] duckdb (read_csv columns/types)
- [x] json
- [x] json representation of spark schema
- [x] jsonschema
- [ ] pandas
- [x] polars
- [ ] protobuf


## Project Structure

```
crates
├── pyaxp               -> Python bindings 
├── yaxp-cli            -> cli version
├── yaxp-common         -> lib (deprecated, replaced by yaxp-core)
└── yaxp-core           -> main lib crate, core

```

## Downloads

- [yaxp-core](https://crates.io/crates/yaxp-core) main lib crate, core
- [yaxp-cli](https://crates.io/crates/yaxp-cli) cli interface on yaxp-core
- [pyaxp](https://pypi.org/project/pyaxp/) python bindings




## TODO

- [x]  pyo3/maturin support
- [ ]  parameter for timezone unit/TZ (testing with polars)
- [x]  support for different xsd file encoding: UTF-16, UTF16LE, ...
- [ ]  more tests
- [ ]  strict schema validation to spec ([xsd](https://www.w3.org/TR/xmlschema11-1/), [avro](https://avro.apache.org/docs/1.11.1/specification/), [json-schema](https://json-schema.org/specification), ...)
- [x]  example implementation [<xsd ⚡> convert](https://xsd-convert.com)
- [x]  option to lowercase column names