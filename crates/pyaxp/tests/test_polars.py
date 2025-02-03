import polars as pl
import pyarrow as pa
import pandas as pd
from pyarrow import csv
from pyaxp import parse_xsd

# Define an Arrow schema
arrow_schema = pa.schema([
    ('id', pa.int64()),
    ('name', pa.string()),
    ('value', pa.float64()),
])

# Convert the Arrow schema to a dictionary that maps column names to their types
# dtype_mapping = {field.name: field.type for field in arrow_schema}
j = parse_xsd("example.xsd", "arrow")
co = csv.ConvertOptions(column_types=j)
padata = csv.read_csv("example-data.csv", read_options=csv.ReadOptions(skip_rows=1), parse_options=csv.ParseOptions(delimiter=";"), convert_options=co)
# Read the CSV file using Polars, specifying the dtype mapping
df = pl.read_csv('data.csv', dtype=dtype_mapping)

print(df)