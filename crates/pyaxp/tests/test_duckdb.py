import duckdb
from pyaxp import parse_xsd

j = parse_xsd("example.xsd", "duckdb")
res = duckdb.sql(f"select * from read_csv('example-data.csv', columns={j})")
