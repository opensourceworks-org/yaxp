from pyspark.sql import SparkSession
from pyspark.sql.types import (
    StructType, StructField, StringType, TimestampType, DateType, DecimalType, IntegerType
)
from pyaxp import parse_xsd

from datetime import datetime, date
from decimal import Decimal

data = [
    ("A1", "B1", "C1", "D1", datetime(2024, 2, 1, 10, 30, 0), date(2024, 2, 1), date(2024, 1, 31),
     "E1", "F1", "G1", "H1", Decimal("123456789012345678.1234567"), "I1", "J1", "K1", "L1",
     date(2024, 2, 1), "M1", "N1", Decimal("100"), 10),

    ("A2", "B2", "C2", None, datetime(2024, 2, 1, 11, 0, 0), None, date(2024, 1, 30),
     "E2", None, "G2", "H2", None, "I2", "J2", "K2", "L2",
     date(2024, 2, 2), "M2", "N2", Decimal("200"), 20),

    ("A3", "B3", "C3", "D3", datetime(2024, 2, 1, 12, 15, 0), date(2024, 2, 3), None,
     "E3", "F3", None, "H3", Decimal("98765432109876543.7654321"), "I3", None, "K3", "L3",
     date(2024, 2, 3), "M3", "N3", None, None)
]

spark = SparkSession.builder.master("local").appName("Test Data").getOrCreate()
schema = parse_xsd("example.xsd", "spark")
df = spark.createDataFrame(data, schema=schema)


def test_parse_schema():
    assert df.schema.fields[4].dataType == TimestampType()
    assert df.schema.fields[5].dataType == DateType()
    assert df.schema.fields[6].dataType == DateType()
    assert df.schema.fields[11].dataType == DecimalType(25, 7)
    assert df.schema.fields[18].dataType == StringType()
    assert df.schema.fields[20].dataType == IntegerType()


def test_read_csv_with_schema():
    assert df.count() == 3
    assert df.collect()[0][4] == datetime(2024, 2, 1, 10, 30, 0)
    assert df.collect()[0][5] == date(2024, 2, 1)
    assert df.collect()[0][6] == date(2024, 1, 31)
    assert df.collect()[0][11] == Decimal("123456789012345678.1234567")
    assert df.collect()[0][18] == "N1"
    assert df.collect()[0][20] == 10
    assert df.collect()[1][4] == datetime(2024, 2, 1, 11, 0, 0)
    assert df.collect()[1][5] == None
    assert df.collect()[1][6] == date(2024, 1, 30)
    assert df.collect()[1][11] == None
    assert df.collect()[1][18] == "N2"
    assert df.collect()[1][20] == 20
    assert df.collect()[2][4] == datetime(2024, 2, 1, 12, 15, 0)
    assert df.collect()[2][5] == date(2024, 2, 3)
    assert df.collect()[2][6] == None
    assert df.collect()[2][11] == Decimal("98765432109876543.7654321")
    assert df.collect()[2][18] == "N3"
    assert df.collect()[2][20] == None


def test_lowercase():
    schema = parse_xsd("example.xsd", "spark", lowercase=True)
    assert len(schema.fields) == 21
    for i in range(len(schema.fields)):
        assert schema.fields[i].name == f"field{i + 1}"
