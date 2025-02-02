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

### with duckdb
```python
$ python
Python 3.12.3 (main, Apr 15 2024, 17:43:11) [Clang 17.0.6 ] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import duckdb
>>> from pyaxp import parse_xsd
>>>
>>> j = parse_xsd("example.xsd", "duckdb")
>>> res = duckdb.sql(f"select * from read_csv('example-data.csv', columns={j})")
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

>>> j
{'Field1': 'VARCHAR(15)', 'Field2': 'VARCHAR(20)', 'Field3': 'VARCHAR(10)', 'Field4': 'VARCHAR(50)', 'Field5': 'TIMESTAMP', 'Field6': 'DATE', 'Field7': 'DATE', 'Field8': 'VARCHAR(10)', 'Field9': 'VARCHAR(3)', 'Field10': 'VARCHAR(30)', 'Field11': 'VARCHAR(10)', 'Field12': 'DECIMAL(25, 7)', 'Field13': 'VARCHAR(255)', 'Field14': 'VARCHAR(255)', 'Field15': 'VARCHAR(255)', 'Field16': 'VARCHAR(255)', 'Field17': 'DATE', 'Field18': 'VARCHAR(30)', 'Field19': 'VARCHAR(255)', 'Field20': 'DECIMAL(25, 7)', 'Field21': 'INTEGER'}
>>>
```

## TODO

- [x] Add pyo3/maturin support
- [ ] Add tests
