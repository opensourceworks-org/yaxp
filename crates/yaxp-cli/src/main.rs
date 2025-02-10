use std::path::PathBuf;
use clap::Parser;
use encoding_rs::{Encoding, UTF_8};
use serde::Serialize;
use yaxp_core::xsdp::parser::parse_file;
use yaxp_core::xsdp::parser::{TimestampOptions, TimestampUnit};

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum OutputFormat {
    #[default]
    Json,
    Arrow,
    Spark,
    JsonSchema,
    Duckdb,
    Polars,
    Avro,
}

/// <yaxp-cli ⚡>
/// Yet Another XSD Parser
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Path to the XSD file
    #[clap(short, long)]
    xsd: PathBuf,

    /// Output format
    #[clap(short, long, default_value = "json")]
    format: OutputFormat,

    /// optional output filename
    #[clap(short, long, default_value = None)]
    output: Option<String>,

    /// optional timeunit
    #[clap(short, long, default_value = "ns")]
    timeunit: TimestampUnit,

    /// optional timezone
    #[clap(short = 'z', long, default_value = "UTC")]
    timezone: String,

    /// optional encoding of the XSD file
    #[clap(short, long, default_value = "utf-8")]
    encoding: String,
}

fn main() {
    let args = Args::parse();

    let timestamp_options = TimestampOptions {
        time_unit: Some(args.timeunit),
        time_zone: Some(args.timezone),
    };

    let use_encoding = Encoding::for_label(args.encoding.as_bytes()).unwrap_or(UTF_8);
    let result = parse_file(args.xsd, Some(timestamp_options), Some(use_encoding));

    match result {
        Ok(schema) => match args.format {
            OutputFormat::Json => {
                if let Some(output) = args.output {
                    schema.write_to_json_file(&output).unwrap();
                    println!("✅ Done!")
                } else {
                    println!("{}", schema.to_json().unwrap());
                }
            }
            OutputFormat::Arrow => {
                let arrow_schema = schema.to_arrow().unwrap();
                println!("{:?}", arrow_schema);
            }
            OutputFormat::Spark => {
                let spark_schema = schema.to_spark().unwrap().to_json().unwrap();
                println!("{}", spark_schema);
            }
            OutputFormat::JsonSchema => {
                let json_schema = schema.to_json_schema();
                println!("{}", json_schema);
            }
            OutputFormat::Duckdb => {
                let duckdb_schema = schema.to_duckdb_schema();
                println!("{:?}", duckdb_schema);
            }
            OutputFormat::Polars => {
                let polars_schema = schema.to_polars();
                println!("{:?}", polars_schema);
            }
            OutputFormat::Avro => {
                let avro_schema = schema.to_avro();
                match avro_schema {
                    Ok(avro_schema) => println!("{}", serde_json::to_string(&avro_schema).unwrap()),
                    Err(e) => eprintln!("❌ {}", e),
                }
            }
        },
        Err(e) => eprintln!("❌ {}", e),
    }
}
