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

    assert df.schema.dtypes()[4] == datatypes.Datetime(time_unit='ns', time_zone=None)


def test_timestampoptions():
    ts_schema = parse_xsd("example.xsd", format="polars",
                        timestamp_options={"time_unit": "us", "time_zone": "UTC"})
    assert ts_schema.get("Field5") == datatypes.Datetime(time_unit='us', time_zone='UTC')

    ts_schema = parse_xsd("example.xsd", format="polars",
                        timestamp_options={"time_unit": "ns", "time_zone": "Europe/Brussels"})
    assert ts_schema.get("Field5") == datatypes.Datetime(time_unit='ns', time_zone='Europe/Brussels')

