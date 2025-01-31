use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use crate::xsdp::xsdp;

pub fn convert_field_to_arrow(field: &str) -> Result<DataType, Box<dyn std::error::Error>> {
    match field {
        "string" => Ok(DataType::Utf8),
        "integer" => Ok(DataType::Int32),
        "decimal" => Ok(DataType::Float64),
        "boolean" => Ok(DataType::Boolean),
        "date" => Ok(DataType::Date32),
        "dateTime" => Ok(DataType::Timestamp(TimeUnit::Nanosecond, None)),

        _ => Ok(DataType::Utf8),
    }
}

// pub fn convert_schema_to_arrow(schema: xsdp::Schema) -> Result<Schema, Box<dyn std::error::Error>> {
//     let mut fields = vec![];
//
//     for element in &schema.schema_element.elements {
//         match &element.data_type {
//             Some(data_type) => {
//                 let field = Field::new(&element.name, convert_field_to_arrow(data_type)?, true);
//                 fields.push(field);
//             }
//             None => {
//                 let schema = convert_schema_to_arrow(element.clone())?;
//                 let field = Field::new(&element.name, DataType::Struct(schema), true);
//                 fields.push(field);
//             }
//         }
//         // let field = Field::new(&element.name, convert_field_to_arrow(&element.data_type)?, true);
//         fields.push(field);
//     }
//
//     Ok(Schema::new(fields))
// }