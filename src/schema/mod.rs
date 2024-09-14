pub mod database;
pub mod themes;
pub mod users;

use database::DatabaseSchema;
use serde::Deserialize;
use serde_valid::Validate;
use themes::ThemesSchema;
use users::UsersSchema;

#[derive(Deserialize, Debug, Validate)]
pub struct RootSchema {
    #[validate]
    pub themes: ThemesSchema,

    pub database: DatabaseSchema,

    pub users: UsersSchema,
}
