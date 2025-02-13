from fastavro import writer, reader, parse_schema
from pyaxp import parse_xsd
from io import BytesIO

# schema = {
#     'doc': 'A weather reading.',
#     'name': 'Weather',
#     'namespace': 'test',
#     'type': 'record',
#     'fields': [
#         {'name': 'station', 'type': 'string'},
#         {'name': 'time', 'type': 'long'},
#         {'name': 'temp', 'type': 'int'},
#     ],
# }


# 'records' can be an iterable (including generator)
# records = [
#     {u'station': u'011990-99999', u'temp': 0, u'time': 1433269388},
#     {u'station': u'011990-99999', u'temp': 22, u'time': 1433270389},
#     {u'station': u'011990-99999', u'temp': -11, u'time': 1433273379},
#     {u'station': u'012650-99999', u'temp': 111, u'time': 1433275478},
# ]

RECORDS = [
    {
        "Field1": "Record1_Value1",
        "Field2": "Record1_Value2",
        "Field3": "Record1_Value3",
        "Field4": "Record1_Optional4",
        "Field5": "Record1_Value5",
        "Field6": "Record1_Optional6",
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
        "Field17": "Record1_Value17",
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
        "Field6": "Rec2_F6",
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
        "Field17": "Rec2_F17",
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
        "Field17": "Test3_F17",
        "Field18": "Test3_F18",
        "Field19": "N",
        "Field20": ".1234",
        "Field21": "Test3_Optional21"
    }
]


# def read_avro_from_list(records, schema):
#     buffer = BytesIO()
#     writer(buffer, schema, records)
#
#     buffer.seek(0)
#
#     return list(reader(buffer))
#
#
def test_avro_schema():
    schema = parse_xsd("example.xsd", "avro")

    parsed_schema = parse_schema(schema)

    assert parsed_schema["__named_schemas"]["Field13"]["symbols"] == ['U', 'N', 'I', 'T']

# def main():
#     test_read_avro_from_list()
#
#
# if __name__ == "__main__":
#     main()
