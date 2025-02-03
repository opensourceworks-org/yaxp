import polars as pl
import pytest
from pyaxp import parse_xsd
import polars.datatypes as datatypes

schema = parse_xsd("example.xsd", "polars")
df = pl.read_csv("example-data.csv", schema=schema)

def test_parse_schema():
    assert df.shape == (3, 21)

    assert df.schema == schema

def test_read_csv_with_schema():

    assert df.schema.dtypes()[4] == datatypes.Datetime(time_unit='ms', time_zone=None)
