use serde::Deserialize;
use serde_valid::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct ThemesSchema {
    #[validate(min_items = 1)]
    pub list: Vec<String>,

    pub default: String,
}
