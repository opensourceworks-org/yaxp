import polars as pl
from pyaxp import parse_xsd


schema = parse_xsd("example.xsd", "schema")
df = pl.read_csv("example-data.csv", schema=schema)
print(df)