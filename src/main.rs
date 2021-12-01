use std::path::Path;

use anyhow::{Error, Result};
use clap::Parser;

pub mod ir;

#[derive(Parser, Debug)]
struct Opts {
    schema: Option<String>,

    #[clap(long, about = "Print output to console")]
    dry: bool,
}

const FALLBACK_SCHEMA_PATHS: &[&str] = &["./schema.prisma", "./prisma/schema.prisma"];

fn load_schema_file(source: &Option<String>) -> Result<String> {
    match source {
        Some(path) => std::fs::read_to_string(path)
            .map_err(|_| Error::msg(format!("schema file not found at {}", path))),
        _ => {
            let path = FALLBACK_SCHEMA_PATHS
                .iter()
                .map(Path::new)
                .find(|path| path.exists() && path.is_file())
                .ok_or_else(|| {
                    Error::msg("schema does not exist at ./schema.prisma or ./prisma/schema.prisma")
                })?;

            Ok(std::fs::read_to_string(path)?)
        }
    }
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let schema_contents = load_schema_file(&opts.schema)?;

    let mut ir = ir::Intermediate::parse(&schema_contents)?;
    ir.transform_names();

    let out = ir.render();

    println!("{}", out);

    Ok(())
}
