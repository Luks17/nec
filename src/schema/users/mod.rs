pub mod extra_field;
pub mod initial_admin;

use extra_field::ExtraFieldSchema;
use initial_admin::InitialAdminSchema;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_valid::Validate;

#[serde_inline_default]
#[derive(Deserialize, Debug, Validate)]
pub struct UsersSchema {
    #[validate(min_items = 1)]
    pub roles: Vec<String>,

    #[serde_inline_default(Vec::new())]
    pub extra_fields: Vec<ExtraFieldSchema>,

    pub initial_admin: InitialAdminSchema,
}
