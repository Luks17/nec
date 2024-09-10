use anyhow::Result;
use nec::{schema::RootSchema, setup::init};
use std::fs::File;

fn main() -> Result<()> {
    init()?;

    let file = File::open("schema.json")?;
    let schema: RootSchema = serde_json::from_reader(file)?;

    println!("{:?}", schema);

    Ok(())
}
