pub mod database;
pub mod entity;
pub mod themes;

use database::DatabaseSchema;
use entity::EntitySchema;
use serde::Deserialize;
use serde_valid::Validate;
use themes::ThemesSchema;

#[derive(Deserialize, Debug, Validate)]
pub struct RootSchema {
    #[validate]
    pub themes: ThemesSchema,

    pub database: DatabaseSchema,

    pub entities: Vec<EntitySchema>,
}
