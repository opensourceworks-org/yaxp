import polars as pl
import pyarrow as pa

# Define an Arrow schema
arrow_schema = pa.schema([
    ('id', pa.int64()),
    ('name', pa.string()),
    ('value', pa.float64()),
])

# Convert the Arrow schema to a dictionary that maps column names to their types
dtype_mapping = {field.name: field.type for field in arrow_schema}

# Read the CSV file using Polars, specifying the dtype mapping
df = pl.read_csv('data.csv', dtype=dtype_mapping)

print(df)