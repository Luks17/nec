use anyhow::Result;
use nec::{gen::Generator, schema::RootSchema, setup::init_project};
use serde_valid::json::FromJsonReader;
use std::fs::File;

fn main() -> Result<()> {
    let project = init_project()?;

    let file = File::open("schema.json")?;
    let schema = RootSchema::from_json_reader(file)?;

    let generator = Generator::new(project, schema);

    generator.generate();

    Ok(())
}
