pub mod attributes;

use attributes::AttributeSchema;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_valid::Validate;

#[serde_inline_default]
#[derive(Deserialize, Debug, Validate)]
pub struct EntitySchema {
    pub name: String,

    #[serde_inline_default(false)]
    pub timestamps: bool,

    pub attributes: Vec<AttributeSchema>,

    pub relations: Vec<RelationSchema>,

    #[serde_inline_default(Option::<ChildEntitiesSchema>::None)]
    pub child_entities: Option<ChildEntitiesSchema>,
}

#[serde_inline_default]
#[derive(Deserialize, Debug)]
pub struct RelationSchema {
    #[serde(rename = "type")]
    pub type_: RelationType,

    pub entity: String,

    #[serde_inline_default(false)]
    pub use_key_as_primary: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    OneToMany,
    OneToOne,
    ManyToOne,
    ManyToMany,
}

#[derive(Deserialize, Debug)]
pub struct ChildEntitiesSchema {
    pub discriminator: String,

    pub list: Vec<ChildEntitySchema>,
}

#[derive(Deserialize, Debug)]
pub struct ChildEntitySchema {
    pub name: String,

    pub attributes: Vec<AttributeSchema>,
}
