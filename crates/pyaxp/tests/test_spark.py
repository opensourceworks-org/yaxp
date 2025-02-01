import json

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
j = parse_xsd("example.xsd", "spark")
spark_schema = StructType.fromJson(json.loads(j))
df = spark.createDataFrame(data, schema=spark_schema)

df.printSchema()
df.schema
df.dtypes

df.show()