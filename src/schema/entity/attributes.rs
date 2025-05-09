use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_valid::Validate;

#[serde_inline_default]
#[derive(Deserialize, Debug, Validate)]
pub struct AttributeSchema {
    pub name: String,

    #[serde_inline_default(false)]
    pub primary: bool,

    #[serde(rename = "type")]
    pub type_: AttributeType,

    #[serde_inline_default(false)]
    pub optional: bool,

    #[serde_inline_default(Option::<u16>::None)]
    pub length: Option<u16>,

    #[serde_inline_default(Option::<u16>::None)]
    pub min: Option<u16>,

    #[serde_inline_default(Option::<u16>::None)]
    pub max: Option<u16>,

    #[serde_inline_default(Option::<bool>::None)]
    pub unique: Option<bool>,

    #[serde_inline_default(Option::<String>::None)]
    pub default: Option<String>,

    #[serde_inline_default(Option::<Vec<EnumItemSchema>>::None)]
    pub enum_items: Option<Vec<EnumItemSchema>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AttributeType {
    String,
    Text,
    Integer,
    Decimal,
    Boolean,
    Date,
    DateTime,
    Enum,
}

#[derive(Deserialize, Debug)]
pub struct EnumItemSchema {
    pub label: String,
    pub value: String,
}
