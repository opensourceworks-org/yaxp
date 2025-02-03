import duckdb
from pyaxp import parse_xsd

j = parse_xsd("example.xsd", "duckdb")
res = duckdb.sql(f"select * from read_csv('example-data.csv', columns={j})")


def test_parse_schema():
    assert res.types[4] == "TIMESTAMP"

def test_read_csv_with_schema():
    assert res.count("*").fetchone()[0] == 3
