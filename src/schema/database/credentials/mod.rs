use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Deserialize, Debug)]
pub struct CredentialsSchema {
    #[serde_inline_default("nec".to_string())]
    pub name: String,

    #[serde_inline_default("localhost".to_string())]
    pub host: String,

    #[serde_inline_default("3306".to_string())]
    pub port: String,

    #[serde_inline_default("root".to_string())]
    pub user: String,

    #[serde_inline_default("123456".to_string())]
    pub password: String,
}
