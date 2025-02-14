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

## Installation

```shell
$ cargo install yaxp-cli
```

## Usage

```shell
$ yaxp-cli --help
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

## Examples

```shell
$ yaxp-cli --xsd example.xsd --format polars
Schema:
name: Field1, field: String
name: Field2, field: String
name: Field3, field: String
name: Field4, field: String
name: Field5, field: Datetime(Milliseconds, None)
name: Field6, field: Date
name: Field7, field: Date
name: Field8, field: String
name: Field9, field: String
name: Field10, field: String
name: Field11, field: String
name: Field12, field: Decimal(Some(25), Some(7))
name: Field13, field: String
name: Field14, field: String
name: Field15, field: String
name: Field16, field: String
name: Field17, field: Date
name: Field18, field: String
name: Field19, field: String
name: Field20, field: Decimal(Some(38), Some(10))
name: Field21, field: Int64

$ yaxp-cli --xsd example.xsd --format arrow
Schema { fields: [Field { name: "Field1", data_type: Utf8, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1", "maxLength": "15"} }, Field { name: "Field2", data_type: Utf8, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: {"maxLength": "20", "maxOccurs": "1"} }, Field { name: "Field3", data_type: Utf8, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: {"maxLength": "10", "maxOccurs": "1"} }, Field { name: "Field4", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxLength": "50", "maxOccurs": "1"} }, Field { name: "Field5", data_type: Timestamp(Nanosecond, None), nullable: false, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1"} }, Field { name: "Field6", data_type: Date32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1"} }, Field { name: "Field7", data_type: Date32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1"} }, Field { name: "Field8", data_type: Utf8, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: {"minLength": "2", "maxOccurs": "1", "maxLength": "10"} }, Field { name: "Field9", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1", "maxLength": "3"} }, Field { name: "Field10", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1", "maxLength": "30"} }, Field { name: "Field11", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1", "maxLength": "10"} }, Field { name: "Field12", data_type: Decimal128(25, 7), nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1"} }, Field { name: "Field13", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1", "values": "N,Q,V,C"} }, Field { name: "Field14", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1", "values": "%,P,R"} }, Field { name: "Field15", data_type: Utf8, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: {"values": "%,P,R", "maxOccurs": "1"} }, Field { name: "Field16", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"pattern": ".{3}", "maxOccurs": "1"} }, Field { name: "Field17", data_type: Date32, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1"} }, Field { name: "Field18", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxLength": "30", "pattern": "[a-cA-C]*", "maxOccurs": "1"} }, Field { name: "Field19", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1", "values": "Y,N"} }, Field { name: "Field20", data_type: Float64, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1"} }, Field { name: "Field21", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {"maxOccurs": "1"} }], metadata: {} }
$ yaxp-cli --xsd example.xsd --format json |jq
{
  "namespace": null,
  "schemaElement": {
    "id": "Main_Element",
    "name": "Main_Element",
    "dataType": null,
    "minOccurs": "1",
    "maxOccurs": "1",
    "minLength": null,
    "maxLength": null,
    "minExclusive": null,
    "maxExclusive": null,
    "minInclusive": null,
    "maxInclusive": null,
    "pattern": null,
    "fractionDigits": null,
    "totalDigits": null,
    "values": null,
    "isCurrency": false,
    "xpath": "Main_Element/Main_Element",
    "nullable": null,
    "elements": [
      {
        "id": "Field1",
        "name": "Field1",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": "15",
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field1",
        "nullable": false,
        "elements": []
      },
      {
        "id": "Field2",
        "name": "Field2",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": "20",
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field2",
        "nullable": false,
        "elements": []
      },
      {
        "id": "Field3",
        "name": "Field3",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": "10",
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field3",
        "nullable": false,
        "elements": []
      },
      {
        "id": "Field4",
        "name": "Field4",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": "50",
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field4",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field5",
        "name": "Field5",
        "dataType": "dateTime",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field5",
        "nullable": false,
        "elements": []
      },
      {
        "id": "Field6",
        "name": "Field6",
        "dataType": "date",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field6",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field7",
        "name": "Field7",
        "dataType": "date",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field7",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field8",
        "name": "Field8",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": "2",
        "maxLength": "10",
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field8",
        "nullable": false,
        "elements": []
      },
      {
        "id": "Field9",
        "name": "Field9",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": "3",
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field9",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field10",
        "name": "Field10",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": "30",
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field10",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field11",
        "name": "Field11",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": "10",
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field11",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field12",
        "name": "Field12",
        "dataType": "decimal",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": "7",
        "totalDigits": "25",
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field12",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field13",
        "name": "Field13",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": [
          "N",
          "Q",
          "V",
          "C"
        ],
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field13",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field14",
        "name": "Field14",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": [
          "%",
          "P",
          "R"
        ],
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field14",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field15",
        "name": "Field15",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": [
          "%",
          "P",
          "R"
        ],
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field15",
        "nullable": false,
        "elements": []
      },
      {
        "id": "Field16",
        "name": "Field16",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": ".{3}",
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field16",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field17",
        "name": "Field17",
        "dataType": "date",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field17",
        "nullable": false,
        "elements": []
      },
      {
        "id": "Field18",
        "name": "Field18",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": "30",
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": "[a-cA-C]*",
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field18",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field19",
        "name": "Field19",
        "dataType": "string",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": [
          "Y",
          "N"
        ],
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field19",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field20",
        "name": "Field20",
        "dataType": "decimal",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field20",
        "nullable": true,
        "elements": []
      },
      {
        "id": "Field21",
        "name": "Field21",
        "dataType": "integer",
        "minOccurs": "1",
        "maxOccurs": "1",
        "minLength": null,
        "maxLength": null,
        "minExclusive": null,
        "maxExclusive": null,
        "minInclusive": null,
        "maxInclusive": null,
        "pattern": null,
        "fractionDigits": null,
        "totalDigits": null,
        "values": null,
        "isCurrency": false,
        "xpath": "Main_Element/Main_Element/Field21",
        "nullable": true,
        "elements": []
      }
    ]
  }
}
$
```


## TODO

- [x]  pyo3/maturin support
- [ ]  parameter for timezone unit/TZ (testing with polars)
- [x]  support for different xsd file encoding: UTF-16, UTF16LE, ...
- [ ]  more tests
- [ ]  strict schema validation to spec ([xsd](https://www.w3.org/TR/xmlschema11-1/), [avro](https://avro.apache.org/docs/1.11.1/specification/), [json-schema](https://json-schema.org/specification), ...)
- [x]  example implementation [<xsd ⚡> convert](https://xsd-convert.com)
- [x]  option to lowercase column names