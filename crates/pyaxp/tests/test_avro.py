import csv
import json
from io import BytesIO
from datetime import datetime

import avro.schema
from avro.datafile import DataFileWriter, DataFileReader
from avro.io import DatumWriter, DatumReader

from pyaxp import parse_xsd


RECORDS = [
    {
        "Field1": "Record1_Value1",
        "Field2": "Record1_Value2",
        "Field3": "Record1_Value3",
        "Field4": "Record1_Optional4",
        "Field5": "Record1_Value5",
        "Field6": "20250213",
        "Field7": None,
        "Field8": "Record1_Value8",
        "Field9": "Record1_Optional9",
        "Field10": None,
        "Field11": "Record1_Optional11",
        "Field12": None,
        "Field13": "N",
        "Field14": "R",
        "Field15": "P",
        "Field16": "Currency explanation",
        "Field17": "20250101",
        "Field18": None,
        "Field19": "Y",
        "Field20": ".05",
        "Field21": "Record1_Optional21"
    },
    {
        "Field1": "Rec2_F1",
        "Field2": "Rec2_F2",
        "Field3": "Rec2_F3",
        "Field4": None,
        "Field5": "Rec2_F5",
        "Field6": "20250112",
        "Field7": "Rec2_F7",
        "Field8": "Rec2_F8",
        "Field9": None,
        "Field10": "Rec2_F10",
        "Field11": None,
        "Field12": "Rec2_F12",
        "Field13": "V",
        "Field14": None,
        "Field15": "P",
        "Field16": None,
        "Field17": "20250101",
        "Field18": "Rec2_F18",
        "Field19": None,
        "Field20": "0.9525",
        "Field21": None
    },
    {
        "Field1": "Test3_F1",
        "Field2": "Test3_F2",
        "Field3": "Test3_F3",
        "Field4": "Test3_Optional4",
        "Field5": "Test3_F5",
        "Field6": None,
        "Field7": "Test3_Optional7",
        "Field8": "Test3_F8",
        "Field9": "Test3_Optional9",
        "Field10": "Test3_Optional10",
        "Field11": "Test3_Optional11",
        "Field12": "Test3_Optional12",
        "Field13": "C",
        "Field14": "P",
        "Field15": "R",
        "Field16": "Detailed currency info",
        "Field17": "20240101",
        "Field18": "Test3_F18",
        "Field19": "N",
        "Field20": ".1234",
        "Field21": "Test3_Optional21"
    }
]


# reading avro from csv is not part of the schema tests

def test_avro_schema():
    schema_dict = parse_xsd("example.xsd", "avro")

    schema_json = json.dumps(schema_dict)
    parsed_schema = avro.schema.parse(schema_json)

    assert parsed_schema.fields[6].type.to_json() == ['null', {'type': 'int', 'logicalType': 'date'}]


# def read_avro_from_list(records, schema):
#     buffer = BytesIO()
#
#     writer = DataFileWriter(buffer, DatumWriter(), schema)
#     for record in records:
#         for field in ["Field6", "Field17"]:
#             date_str = record[field]
#             d = datetime.strptime(date_str.strip(), "%Y%m%d")
#             # record["Field6"] = int(record["Field6"]) if record["Field6"] is not None else None
#             record[field] = d
#             print(record)
#             writer.append(record)
#     writer.flush()
#
#     avro_bytes = buffer.getvalue()
#     writer.close()
#
#     new_buffer = BytesIO(avro_bytes)
#     reader = DataFileReader(new_buffer, DatumReader())
#     result = list(reader)
#     reader.close()
#
#     return result


# def test_read_avro_from_list():
#     schema_dict = parse_xsd("example_clean_avro.xsd", "avro")
#
#     schema_json = json.dumps(schema_dict)
#     parsed_schema = avro.schema.parse(schema_json)
#
#     test_data = read_avro_from_list(RECORDS, parsed_schema)
#
#     assert test_data == RECORDS
#
# def read_csv_data_as_avro(csv_file_path, schema):
#
#     records = []
#     with open(csv_file_path, newline='') as csvfile:
#         csv_reader = csv.DictReader(csvfile, delimiter=';')
#         for row in csv_reader:
#             records.append(row)
#
#     buffer = BytesIO()
#     writer = DataFileWriter(buffer, DatumWriter(), schema)
#     for record in records:
#         writer.append(record)
#     writer.flush()
#     avro_bytes = buffer.getvalue()
#     writer.close()
#
#     new_buffer = BytesIO(avro_bytes)
#     reader = DataFileReader(new_buffer, DatumReader())
#     avro_records = list(reader)
#     reader.close()
#
#     return avro_records

# def test_read_csv_data_as_avro():
#     schema_dict = parse_xsd("example_clean_avro.xsd", "avro")
#
#     schema_json = json.dumps(schema_dict)
#     parsed_schema = avro.schema.parse(schema_json)
#
#     avro_records = read_csv_data_as_avro("example-data-avro.csv", parsed_schema)
#
#
#     assert avro_records[0]["Field14"] == "P"
#     assert len(avro_records) == 3

# def main():
#     # test_read_avro_from_list()
#     test_read_csv_data_as_avro()
#
#
#
# if __name__ == "__main__":
#     main()
