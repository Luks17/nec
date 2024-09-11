pub mod database;
pub mod themes;

use database::DatabaseSchema;
use serde::Deserialize;
use themes::ThemesSchema;

#[derive(Deserialize, Debug)]
pub struct RootSchema {
    pub themes: ThemesSchema,
    pub database: DatabaseSchema,
}
