use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_valid::Validate;

#[serde_inline_default]
#[derive(Deserialize, Debug, Validate)]
pub struct InitialAdminSchema {
    pub name: String,
    pub password: String,

    #[serde_inline_default(Vec::new())]
    pub extra: Vec<String>,
}
