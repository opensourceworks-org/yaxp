import pyarrow as pa
from pyarrow import csv
from pyaxp import parse_xsd

arrow_schema = parse_xsd("example.xsd", "arrow")
convert_options = csv.ConvertOptions(column_types=arrow_schema)
arrow_df = csv.read_csv("example-data.csv",
                        parse_options=csv.ParseOptions(delimiter=";"),
                        convert_options=convert_options)

print(arrow_df)