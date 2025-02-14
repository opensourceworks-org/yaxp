<p align="center">
  <a href="https://pypi.org/project/pyaxp/">
    <img alt="version" src="https://img.shields.io/pypi/v/pyaxp.svg">
  </a>  
  <a href="https://pypi.org/project/pyaxp/">
    <img alt="downloads" src="https://img.shields.io/pypi/dm/pyaxp">
  </a>
  <a href="https://pypi.org/project/pyaxp/">
    <img alt="pipelines" src="https://img.shields.io/github/actions/workflow/status/opensourceworks-org/yaxp/pyaxp-ci.yml?logo=github">
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

## User Guide
### Python
- create and activate a Python virtual environment (or use poetry, uv, etc.)
- install pyaxp 

```shell
(venv) $ uv pip install pyaxp
Using Python 3.12.3 environment at venv
Resolved 1 package in 323ms
Prepared 1 package in 140ms
Installed 1 package in 2ms
 + pyaxp==0.1.6
(venv) $ 
```

```python
Python 3.12.3 (main, Apr 15 2024, 17:43:11) [Clang 17.0.6 ] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyspark.sql import SparkSession
... from pyaxp import parse_xsd
...
... from datetime import datetime, date
... from decimal import Decimal
...
... data = [
    ...     ("A1", "B1", "C1", "D1", datetime(2024, 2, 1, 10, 30, 0), date(2024, 2, 1), date(2024, 1, 31),
             ...      "E1", "F1", "G1", "H1", Decimal("123456789012345678.1234567"), "I1", "J1", "K1", "L1",
    ...      date(2024, 2, 1), "M1", "N1", Decimal("100"), 10),
...
...     ("A2", "B2", "C2", None, datetime(2024, 2, 1, 11, 0, 0), None, date(2024, 1, 30),
         ...      "E2", None, "G2", "H2", None, "I2", "J2", "K2", "L2",
...      date(2024, 2, 2), "M2", "N2", Decimal("200"), 20),
...
...     ("A3", "B3", "C3", "D3", datetime(2024, 2, 1, 12, 15, 0), date(2024, 2, 3), None,
         ...      "E3", "F3", None, "H3", Decimal("98765432109876543.7654321"), "I3", None, "K3", "L3",
...      date(2024, 2, 3), "M3", "N3", None, None)
... ]
...
...
... spark = SparkSession.builder.master("local").appName("Test Data").getOrCreate()
... schema = parse_xsd("example.xsd", "spark")
... df = spark.createDataFrame(data, schema=schema)
...
25/02/08 13:22:01 WARN Utils: Your hostname, Jeroens-MacBook-Air.local resolves to a loopback address: 127.0.0.1; using 192.168.69.217 instead (on interface en0)
25/02/08 13:22:01 WARN Utils: Set SPARK_LOCAL_IP if you need to bind to another address
Setting default log level to "WARN".
To adjust logging level use sc.setLogLevel(newLevel). For SparkR, use setLogLevel(newLevel).
25/02/08 13:22:01 WARN NativeCodeLoader: Unable to load native-hadoop library for your platform... using builtin-java classes where applicable
>>> type(schema)
<class 'pyspark.sql.types.StructType'>
>>> sch25/02/08 13:22:15 WARN GarbageCollectionMetrics: To enable non-built-in garbage collector(s) List(G1 Concurrent GC), users should configure it(them) to spark.eventLog.gcMetrics.youngGenerationGarbageCollectors or spark.eventLog.gcMetrics.oldGenerationGarbageCollectors
>>> schema
StructType([StructField('Field1', StringType(), False), StructField('Field2', StringType(), False), StructField('Field3', StringType(), False), StructField('Field4', StringType(), True), StructField('Field5', TimestampType(), False), StructField('Field6', DateType(), True), StructField('Field7', DateType(), True), StructField('Field8', StringType(), False), StructField('Field9', StringType(), True), StructField('Field10', StringType(), True), StructField('Field11', StringType(), True), StructField('Field12', DecimalType(25,7), True), StructField('Field13', StringType(), True), StructField('Field14', StringType(), True), StructField('Field15', StringType(), False), StructField('Field16', StringType(), True), StructField('Field17', DateType(), False), StructField('Field18', StringType(), True), StructField('Field19', StringType(), True), StructField('Field20', DecimalType(10,0), True), StructField('Field21', IntegerType(), True)])
>>> df
DataFrame[Field1: string, Field2: string, Field3: string, Field4: string, Field5: timestamp, Field6: date, Field7: date, Field8: string, Field9: string, Field10: string, Field11: string, Field12: decimal(25,7), Field13: string, Field14: string, Field15: string, Field16: string, Field17: date, Field18: string, Field19: string, Field20: decimal(10,0), Field21: int]
>>> df.show()
+------+------+------+------+-------------------+----------+----------+------+------+-------+-------+--------------------+-------+-------+-------+-------+----------+-------+-------+-------+-------+
|Field1|Field2|Field3|Field4|             Field5|    Field6|    Field7|Field8|Field9|Field10|Field11|             Field12|Field13|Field14|Field15|Field16|   Field17|Field18|Field19|Field20|Field21|
+------+------+------+------+-------------------+----------+----------+------+------+-------+-------+--------------------+-------+-------+-------+-------+----------+-------+-------+-------+-------+
|    A1|    B1|    C1|    D1|2024-02-01 10:30:00|2024-02-01|2024-01-31|    E1|    F1|     G1|     H1|12345678901234567...|     I1|     J1|     K1|     L1|2024-02-01|     M1|     N1|    100|     10|
|    A2|    B2|    C2|  NULL|2024-02-01 11:00:00|      NULL|2024-01-30|    E2|  NULL|     G2|     H2|                NULL|     I2|     J2|     K2|     L2|2024-02-02|     M2|     N2|    200|     20|
|    A3|    B3|    C3|    D3|2024-02-01 12:15:00|2024-02-03|      NULL|    E3|    F3|   NULL|     H3|98765432109876543...|     I3|   NULL|     K3|     L3|2024-02-03|     M3|     N3|   NULL|   NULL|
+------+------+------+------+-------------------+----------+----------+------+------+-------+-------+--------------------+-------+-------+-------+-------+----------+-------+-------+-------+-------+


>>> df.printSchema()
root
 |-- Field1: string (nullable = false)
 |-- Field2: string (nullable = false)
 |-- Field3: string (nullable = false)
 |-- Field4: string (nullable = true)
 |-- Field5: timestamp (nullable = false)
 |-- Field6: date (nullable = true)
 |-- Field7: date (nullable = true)
 |-- Field8: string (nullable = false)
 |-- Field9: string (nullable = true)
 |-- Field10: string (nullable = true)
 |-- Field11: string (nullable = true)
 |-- Field12: decimal(25,7) (nullable = true)
 |-- Field13: string (nullable = true)
 |-- Field14: string (nullable = true)
 |-- Field15: string (nullable = false)
 |-- Field16: string (nullable = true)
 |-- Field17: date (nullable = false)
 |-- Field18: string (nullable = true)
 |-- Field19: string (nullable = true)
 |-- Field20: decimal(10,0) (nullable = true)
 |-- Field21: integer (nullable = true)

>>> df.schema
StructType([StructField('Field1', StringType(), False), StructField('Field2', StringType(), False), StructField('Field3', StringType(), False), StructField('Field4', StringType(), True), StructField('Field5', TimestampType(), False), StructField('Field6', DateType(), True), StructField('Field7', DateType(), True), StructField('Field8', StringType(), False), StructField('Field9', StringType(), True), StructField('Field10', StringType(), True), StructField('Field11', StringType(), True), StructField('Field12', DecimalType(25,7), True), StructField('Field13', StringType(), True), StructField('Field14', StringType(), True), StructField('Field15', StringType(), False), StructField('Field16', StringType(), True), StructField('Field17', DateType(), False), StructField('Field18', StringType(), True), StructField('Field19', StringType(), True), StructField('Field20', DecimalType(10,0), True), StructField('Field21', IntegerType(), True)])
>>> df.dtypes
[('Field1', 'string'), ('Field2', 'string'), ('Field3', 'string'), ('Field4', 'string'), ('Field5', 'timestamp'), ('Field6', 'date'), ('Field7', 'date'), ('Field8', 'string'), ('Field9', 'string'), ('Field10', 'string'), ('Field11', 'string'), ('Field12', 'decimal(25,7)'), ('Field13', 'string'), ('Field14', 'string'), ('Field15', 'string'), ('Field16', 'string'), ('Field17', 'date'), ('Field18', 'string'), ('Field19', 'string'), ('Field20', 'decimal(10,0)'), ('Field21', 'int')]
>>>
>>> df.show()
+------+------+------+------+-------------------+----------+----------+------+------+-------+-------+--------------------+-------+-------+-------+-------+----------+-------+-------+-------+-------+
|Field1|Field2|Field3|Field4|             Field5|    Field6|    Field7|Field8|Field9|Field10|Field11|             Field12|Field13|Field14|Field15|Field16|   Field17|Field18|Field19|Field20|Field21|
+------+------+------+------+-------------------+----------+----------+------+------+-------+-------+--------------------+-------+-------+-------+-------+----------+-------+-------+-------+-------+
|    A1|    B1|    C1|    D1|2024-02-01 10:30:00|2024-02-01|2024-01-31|    E1|    F1|     G1|     H1|12345678901234567...|     I1|     J1|     K1|     L1|2024-02-01|     M1|     N1|    100|     10|
|    A2|    B2|    C2|  NULL|2024-02-01 11:00:00|      NULL|2024-01-30|    E2|  NULL|     G2|     H2|                NULL|     I2|     J2|     K2|     L2|2024-02-02|     M2|     N2|    200|     20|
|    A3|    B3|    C3|    D3|2024-02-01 12:15:00|2024-02-03|      NULL|    E3|    F3|   NULL|     H3|98765432109876543...|     I3|   NULL|     K3|     L3|2024-02-03|     M3|     N3|   NULL|   NULL|
+------+------+------+------+-------------------+----------+----------+------+------+-------+-------+--------------------+-------+-------+-------+-------+----------+-------+-------+-------+-------+

>>>
```

### with duckdb
```python
$ python
Python 3.12.3 (main, Apr 15 2024, 17:43:11) [Clang 17.0.6 ] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import duckdb
>>> from pyaxp import parse_xsd
>>>
>>> duckdb_schema = parse_xsd("example.xsd", format="duckdb")
>>> type(duckdb_schema)
<class 'dict'>
>>> res = duckdb.sql(f"select * from read_csv('example-data.csv', columns={duckdb_schema})")
>>> res
┌─────────┬─────────┬─────────┬─────────┬─────────────────────┬────────────┬────────────┬─────────┬───┬─────────┬─────────┬─────────┬─────────┬────────────┬─────────┬─────────┬───────────────┬─────────┐
│ Field1  │ Field2  │ Field3  │ Field4  │       Field5        │   Field6   │   Field7   │ Field8  │ … │ Field13 │ Field14 │ Field15 │ Field16 │  Field17   │ Field18 │ Field19 │    Field20    │ Field21 │
│ varchar │ varchar │ varchar │ varchar │      timestamp      │    date    │    date    │ varchar │   │ varchar │ varchar │ varchar │ varchar │    date    │ varchar │ varchar │ decimal(25,7) │  int32  │
├─────────┼─────────┼─────────┼─────────┼─────────────────────┼────────────┼────────────┼─────────┼───┼─────────┼─────────┼─────────┼─────────┼────────────┼─────────┼─────────┼───────────────┼─────────┤
│ A1      │ B1      │ C1      │ D1      │ 2024-02-01 09:30:00 │ 2024-02-01 │ 2024-01-31 │ E1      │ … │ I1      │ J1      │ K1      │ L1      │ 2024-02-01 │ M1      │ N1      │   100.0000000 │      10 │
│ A2      │ B2      │ C2      │ NULL    │ 2024-02-01 10:00:00 │ NULL       │ 2024-01-30 │ E2      │ … │ I2      │ J2      │ K2      │ L2      │ 2024-02-02 │ M2      │ N2      │   200.0000000 │      20 │
│ A3      │ B3      │ C3      │ D3      │ 2024-02-01 11:15:00 │ 2024-02-03 │ NULL       │ E3      │ … │ I3      │ NULL    │ K3      │ L3      │ 2024-02-03 │ M3      │ N3      │          NULL │    NULL │
├─────────┴─────────┴─────────┴─────────┴─────────────────────┴────────────┴────────────┴─────────┴───┴─────────┴─────────┴─────────┴─────────┴────────────┴─────────┴─────────┴───────────────┴─────────┤
│ 3 rows                                                                                                                                                                           21 columns (17 shown) │
└────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

>>> duckdb_schema
{'Field1': 'VARCHAR(15)', 'Field2': 'VARCHAR(20)', 'Field3': 'VARCHAR(10)', 'Field4': 'VARCHAR(50)', 'Field5': 'TIMESTAMP', 'Field6': 'DATE', 'Field7': 'DATE', 'Field8': 'VARCHAR(10)', 'Field9': 'VARCHAR(3)', 'Field10': 'VARCHAR(30)', 'Field11': 'VARCHAR(10)', 'Field12': 'DECIMAL(25, 7)', 'Field13': 'VARCHAR(255)', 'Field14': 'VARCHAR(255)', 'Field15': 'VARCHAR(255)', 'Field16': 'VARCHAR(255)', 'Field17': 'DATE', 'Field18': 'VARCHAR(30)', 'Field19': 'VARCHAR(255)', 'Field20': 'DECIMAL(25, 7)', 'Field21': 'INTEGER'}
>>>
```


### with pyarrow
```python
>>> import pyarrow as pa
>>> from pyarrow import csv
>>> from pyaxp import parse_xsd
>>>
>>> arrow_schema = parse_xsd("example.xsd", format="arrow")
>>> type(arrow_schema)
<class 'pyarrow.lib.Schema'>
>>> convert_options = csv.ConvertOptions(column_types=arrow_schema)
>>> arrow_df = csv.read_csv("example-data.csv",
...                         parse_options=csv.ParseOptions(delimiter=";"),
...                         convert_options=convert_options)
>>>
>>> print(arrow_df)
pyarrow.Table
Field1: string
Field2: string
Field3: string
Field4: string
Field5: timestamp[ns]
Field6: date32[day]
Field7: date32[day]
Field8: string
Field9: string
Field10: string
Field11: string
Field12: decimal128(25, 7)
Field13: string
Field14: string
Field15: string
Field16: string
Field17: date32[day]
Field18: string
Field19: string
Field20: double
Field21: int32
----
Field1: [["A1","A2","A3"]]
Field2: [["B1","B2","B3"]]
Field3: [["C1","C2","C3"]]
Field4: [["D1","","D3"]]
Field5: [[2024-02-01 10:30:00.000000000,2024-02-01 11:00:00.000000000,2024-02-01 12:15:00.000000000]]
Field6: [[2024-02-01,null,2024-02-03]]
Field7: [[2024-01-31,2024-01-30,null]]
Field8: [["E1","E2","E3"]]
Field9: [["F1","","F3"]]
Field10: [["G1","G2",""]]
...
>>> print(arrow_df.to_struct_array())
[
  -- is_valid: all not null
  -- child 0 type: string
    [
      "A1",
      "A2",
      "A3"
    ]
  -- child 1 type: string
    [
      "B1",
      "B2",
      "B3"
    ]
  -- child 2 type: string
    [
      "C1",
      "C2",
      "C3"
    ]
  -- child 3 type: string
    [
      "D1",
      "",
      "D3"
    ]
  -- child 4 type: timestamp[ns]
    [
      2024-02-01 10:30:00.000000000,
      2024-02-01 11:00:00.000000000,
      2024-02-01 12:15:00.000000000
    ]
  -- child 5 type: date32[day]
    [
      2024-02-01,
      null,
      2024-02-03
    ]
  -- child 6 type: date32[day]
    [
      2024-01-31,
      2024-01-30,
      null
    ]
  -- child 7 type: string
    [
      "E1",
      "E2",
      "E3"
    ]
  -- child 8 type: string
    [
      "F1",
      "",
      "F3"
    ]
  -- child 9 type: string
    [
      "G1",
      "G2",
      ""
    ]
  -- child 10 type: string
    [
      "H1",
      "H2",
      "H3"
    ]
  -- child 11 type: decimal128(25, 7)
    [
      123456789012345678.1234567,
      null,
      98765432109876543.7654321
    ]
  -- child 12 type: string
    [
      "I1",
      "I2",
      "I3"
    ]
  -- child 13 type: string
    [
      "J1",
      "J2",
      ""
    ]
  -- child 14 type: string
    [
      "K1",
      "K2",
      "K3"
    ]
  -- child 15 type: string
    [
      "L1",
      "L2",
      "L3"
    ]
  -- child 16 type: date32[day]
    [
      2024-02-01,
      2024-02-02,
      2024-02-03
    ]
  -- child 17 type: string
    [
      "M1",
      "M2",
      "M3"
    ]
  -- child 18 type: string
    [
      "N1",
      "N2",
      "N3"
    ]
  -- child 19 type: double
    [
      100,
      200,
      null
    ]
  -- child 20 type: int32
    [
      10,
      20,
      null
    ]
]
>>>
```

### with polars
```python
>> import polars as pl
>>> from pyaxp import parse_xsd
>>> schema = parse_xsd("example.xsd", format="polars")
>>> type(schema)
<class 'dict'>
>>> df = pl.read_csv("example-data.csv", schema=schema, separator=";")
>>> df
shape: (3, 21)
┌────────┬────────┬────────┬────────┬───┬─────────┬─────────┬────────────────┬─────────┐
│ Field1 ┆ Field2 ┆ Field3 ┆ Field4 ┆ … ┆ Field18 ┆ Field19 ┆ Field20        ┆ Field21 │
│ ---    ┆ ---    ┆ ---    ┆ ---    ┆   ┆ ---     ┆ ---     ┆ ---            ┆ ---     │
│ str    ┆ str    ┆ str    ┆ str    ┆   ┆ str     ┆ str     ┆ decimal[38,10] ┆ i64     │
╞════════╪════════╪════════╪════════╪═══╪═════════╪═════════╪════════════════╪═════════╡
│ A1     ┆ B1     ┆ C1     ┆ D1     ┆ … ┆ M1      ┆ Y       ┆ 100.0000000000 ┆ 10      │
│ A2     ┆ B2     ┆ C2     ┆ null   ┆ … ┆ M2      ┆ N       ┆ 200.0000000000 ┆ 20      │
│ A3     ┆ B3     ┆ C3     ┆ D3     ┆ … ┆ M3      ┆ Y       ┆ null           ┆ null    │
└────────┴────────┴────────┴────────┴───┴─────────┴─────────┴────────────────┴─────────┘
>>> df.types
Traceback (most recent call last):
File "<python-input-7>", line 1, in <module>
df.types
AttributeError: 'DataFrame' object has no attribute 'types'. Did you mean: 'dtypes'?
>>> df.dtypes
[String, String, String, String, Datetime(time_unit='ns', time_zone=None), Date, Date, String, String, String, String, Decimal(precision=25, scale=7), String, String, String, String, Date, String, String, Decimal(precision=38, scale=10), Int64]
>>> schema
{'Field1': String, 'Field2': String, 'Field3': String, 'Field4': String, 'Field5': Datetime(time_unit='ns', time_zone=None), 'Field6': Date, 'Field7': Date, 'Field8': String, 'Field9': String, 'Field10': String, 'Field11': String, 'Field12': Decimal(precision=25, scale=7), 'Field13': String, 'Field14': String, 'Field15': String, 'Field16': String, 'Field17': Date, 'Field18': String, 'Field19': String, 'Field20': Decimal(precision=38, scale=10), 'Field21': Int64}
>>>
```

### with avro
```python
>>> schema = parse_xsd("example.xsd", "avro")
>>> type(schema)
<class 'dict'>
>>> schema
{'type': 'record', 'name': 'Main_Element', 'doc': None, 'aliases': None, 'fields': [{'name': 'Field1', 'type': 'string', 'doc': None}, {'name': 'Field2', 'type': 'string', 'doc': None}, {'name': 'Field3', 'type': 'string', 'doc': None}, {'name': 'Field4', 'type': ['null', 'string'], 'doc': None}, {'name': 'Field5', 'type': 'string', 'doc': None}, {'name': 'Field6', 'type': ['null', {'type': 'int', 'logicalType': 'date'}], 'doc': None}, {'name': 'Field7', 'type': ['null', {'type': 'int', 'logicalType': 'date'}], 'doc': None}, {'name': 'Field8', 'type': 'string', 'doc': None}, {'name': 'Field9', 'type': ['null', 'string'], 'doc': None}, {'name': 'Field10', 'type': ['null', 'string'], 'doc': None}, {'name': 'Field11', 'type': ['null', 'string'], 'doc': None}, {'name': 'Field12', 'type': ['null', 'string'], 'doc': None}, {'name': 'Field13', 'type': ['null', {'type': 'enum', 'doc': None, 'name': 'Field13', 'symbols': ['U', 'N', 'I', 'T'], 'namespace': None}], 'doc': None}, {'name': 'Field14', 'type': ['null', {'type': 'enum', 'doc': None, 'name': 'Field14', 'symbols': ['PCT', 'R', 'D'], 'namespace': None}], 'doc': None}, {'name': 'Field15', 'type': {'type': 'enum', 'doc': None, 'name': 'Field15', 'symbols': ['PCT', 'R', 'D'], 'namespace': None}, 'doc': None}, {'name': 'Field16', 'type': ['null', 'string'], 'doc': 'explanation about the currency type'}, {'name': 'Field17', 'type': {'type': 'int', 'logicalType': 'date'}, 'doc': None}, {'name': 'Field18', 'type': ['null', 'string'], 'doc': None}, {'name': 'Field19', 'type': ['null', {'type': 'enum', 'doc': None, 'name': 'Field19', 'symbols': ['Y', 'N'], 'namespace': None}], 'doc': None}, {'name': 'Field20', 'type': ['null', 'string'], 'doc': 'percentage (ie. .08 -> 8% and .7523 -> 72.23%)'}, {'name': 'Field21', 'type': ['null', 'string'], 'doc': None}], 'namespace': None}
>>> import json
>>> print(json.dumps(schema, indent=4))
{
    "type": "record",
    "name": "Main_Element",
    "doc": null,
    "aliases": null,
    "fields": [
        {
            "name": "Field1",
            "type": "string",
            "doc": null
        },
        {
            "name": "Field2",
            "type": "string",
            "doc": null
        },
        {
            "name": "Field3",
            "type": "string",
            "doc": null
        },
        {
            "name": "Field4",
            "type": [
                "null",
                "string"
            ],
            "doc": null
        },
        {
            "name": "Field5",
            "type": "string",
            "doc": null
        },
        {
            "name": "Field6",
            "type": [
                "null",
                {
                    "type": "int",
                    "logicalType": "date"
                }
            ],
            "doc": null
        },
        {
            "name": "Field7",
            "type": [
                "null",
                {
                    "type": "int",
                    "logicalType": "date"
                }
            ],
            "doc": null
        },
        {
            "name": "Field8",
            "type": "string",
            "doc": null
        },
        {
            "name": "Field9",
            "type": [
                "null",
                "string"
            ],
            "doc": null
        },
        {
            "name": "Field10",
            "type": [
                "null",
                "string"
            ],
            "doc": null
        },
        {
            "name": "Field11",
            "type": [
                "null",
                "string"
            ],
            "doc": null
        },
        {
            "name": "Field12",
            "type": [
                "null",
                "string"
            ],
            "doc": null
        },
        {
            "name": "Field13",
            "type": [
                "null",
                {
                    "type": "enum",
                    "doc": null,
                    "name": "Field13",
                    "symbols": [
                        "U",
                        "N",
                        "I",
                        "T"
                    ],
                    "namespace": null
                }
            ],
            "doc": null
        },
        {
            "name": "Field14",
            "type": [
                "null",
                {
                    "type": "enum",
                    "doc": null,
                    "name": "Field14",
                    "symbols": [
                        "PCT",
                        "R",
                        "D"
                    ],
                    "namespace": null
                }
            ],
            "doc": null
        },
        {
            "name": "Field15",
            "type": {
                "type": "enum",
                "doc": null,
                "name": "Field15",
                "symbols": [
                    "PCT",
                    "R",
                    "D"
                ],
                "namespace": null
            },
            "doc": null
        },
        {
            "name": "Field16",
            "type": [
                "null",
                "string"
            ],
            "doc": "explanation about the currency type"
        },
        {
            "name": "Field17",
            "type": {
                "type": "int",
                "logicalType": "date"
            },
            "doc": null
        },
        {
            "name": "Field18",
            "type": [
                "null",
                "string"
            ],
            "doc": null
        },
        {
            "name": "Field19",
            "type": [
                "null",
                {
                    "type": "enum",
                    "doc": null,
                    "name": "Field19",
                    "symbols": [
                        "Y",
                        "N"
                    ],
                    "namespace": null
                }
            ],
            "doc": null
        },
        {
            "name": "Field20",
            "type": [
                "null",
                "string"
            ],
            "doc": "percentage, ie.: .08 -> 8%"
        },
        {
            "name": "Field21",
            "type": [
                "null",
                "string"
            ],
            "doc": null
        }
    ],
    "namespace": null
}
>>>
```


## TODO

- [x]  pyo3/maturin support
- [ ]  parameter for timezone unit/TZ (testing with polars)
- [x]  support for different xsd file encoding: UTF-16, UTF16LE, ...
- [ ]  more tests
- [ ]  strict schema validation to spec ([xsd](https://www.w3.org/TR/xmlschema11-1/), [avro](https://avro.apache.org/docs/1.11.1/specification/), [json-schema](https://json-schema.org/specification), ...)
- [x]  example implementation [<xsd ⚡> convert](https://xsd-convert.com)
- [x]  option to lowercase column names
