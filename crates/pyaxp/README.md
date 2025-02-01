<p align="center">
  <a href="https://pypi.org/project/pyaxp/">
    <img alt="downloads" src="https://img.shields.io/pypi/dm/pyaxp">
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
>>> import json
>>>
>>> from pyspark.sql import SparkSession
>>> from pyspark.sql.types import (
...     StructType, StructField, StringType, TimestampType, DateType, DecimalType, IntegerType
... )
>>> from pyaxp import parse_xsd
>>>
>>> from datetime import datetime, date
>>> from decimal import Decimal
>>>
>>> data = [
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
>>>
>>>
>>> spark = SparkSession.builder.master("local").appName("Test Data").getOrCreate()
25/02/01 16:27:30 WARN Utils: Set SPARK_LOCAL_IP if you need to bind to another address
Setting default log level to "WARN".
To adjust logging level use sc.setLogLevel(newLevel). For SparkR, use setLogLevel(newLevel).
25/02/01 16:27:30 WARN NativeCodeLoader: Unable to load native-hadoop library for your platform... using builtin-java classes where applicable
>>> 25/02/01 16:27:42 WARN GarbageCollectionMetrics: To enable non-built-in garbage collector(s) List(G1 Concurrent GC), users should configure it(them) to spark.eventLog.gcMetrics.youngGenerationGarbageCollectors or spark.eventLog.gcMetrics.oldGenerationGarbageCollectors

>>> j = parse_xsd("example.xsd", "spark")
>>> spark_schema = StructType.fromJson(json.loads(j))
>>> df = spark.createDataFrame(data, schema=spark_schema)
>>>
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


## TODO

- [x] Add pyo3/maturin support
- [ ] Add tests
