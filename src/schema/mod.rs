pub mod database;

use database::DatabaseSchema;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RootSchema {
    pub database: DatabaseSchema,
}
