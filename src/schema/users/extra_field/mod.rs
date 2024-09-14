use serde::Deserialize;
use serde_valid::Validate;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Number,
    Phone,
    Cpf,
    Email,
}

#[derive(Deserialize, Debug, Validate)]
pub struct ExtraFieldSchema {
    pub name: String,

    pub field_type: FieldType,
}
