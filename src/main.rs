use anyhow::Result;
use nec::{schema::RootSchema, setup::init};
use serde_valid::json::FromJsonReader;
use std::fs::File;

fn main() -> Result<()> {
    init()?;

    let file = File::open("schema.json")?;
    let schema: RootSchema = RootSchema::from_json_reader(file)?;

    println!("{:?}", schema);

    Ok(())
}
