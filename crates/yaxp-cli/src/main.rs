use clap::Parser;
use serde::Serialize;
use yaxp_common::xsdp::parser::parse_file;
use yaxp_common::xsdp::parser::{TimestampUnit, TimestampOptions};

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
}

/// <yaxp-cli ⚡>
/// Yet Another XSD Parser
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Path to the XSD file
    #[clap(short, long)]
    xsd: String,

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
}

fn main() {
    let args = Args::parse();

    let timestamp_options = TimestampOptions {
        time_unit: Some(args.timeunit),
        time_zone: Some(args.timezone),
    };

    let result = parse_file(&args.xsd, Some(timestamp_options));

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
                // dbg!(arrow_schema);
                println!("{:?}", arrow_schema);
            },
            OutputFormat::Spark => {
                let spark_schema = schema.to_spark().unwrap().to_json().unwrap();
                println!("{}", spark_schema);
            },
            OutputFormat::JsonSchema => {
                let json_schema = schema.to_json_schema();
                println!("{}", json_schema);
            },
            OutputFormat::Duckdb => {
                let duckdb_schema = schema.to_duckdb_schema();
                println!("{:?}", duckdb_schema);
            },
            OutputFormat::Polars => {
                let polars_schema = schema.to_polars();
                println!("{:?}", polars_schema);
            },
        },
        Err(e) => eprintln!("❌ {}", e),
    }
}
