use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ThemesSchema {
    pub list: Vec<String>,
    pub default: String,
}
