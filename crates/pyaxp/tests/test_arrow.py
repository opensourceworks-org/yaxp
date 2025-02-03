import pyarrow as pa
from pyarrow import csv
from pyaxp import parse_xsd


arrow_schema = parse_xsd("example.xsd", "arrow")
convert_options = csv.ConvertOptions(column_types=arrow_schema)
arrow_df = csv.read_csv("example-data.csv",
                        parse_options=csv.ParseOptions(delimiter=";"),
                        convert_options=convert_options)

def test_parse_schema():
    f = arrow_schema.field(4)
    assert f.type == pa.timestamp("ns")
    assert arrow_df.num_columns == 21

def test_read_csv_with_schema():
    assert arrow_df.schema.field(4).type == pa.timestamp("ns")
    assert arrow_df.num_rows == 3