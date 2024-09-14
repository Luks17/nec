pub mod credentials;

use credentials::CredentialsSchema;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_valid::Validate;

#[serde_inline_default]
#[derive(Deserialize, Debug, Validate)]
pub struct DatabaseSchema {
    #[serde_inline_default(false)]
    pub use_docker: bool,

    #[serde(default)]
    pub credentials: CredentialsSchema,
}
