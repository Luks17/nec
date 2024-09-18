pub mod user;

use crate::core::react::{self, React};
use genco::{quote_fn, tokens::FormatInto};
use serde::Serialize;
use serde_valid::{json::ToJsonString, Validate};
use std::fmt::Display;

#[derive(Serialize, Validate)]
struct TypeOrmColumnOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<u16>,
}

impl TypeOrmColumnOptions {
    pub fn new() -> Self {
        TypeOrmColumnOptions {
            type_: None,
            unique: None,
            length: None,
        }
    }
}

impl Display for TypeOrmColumnOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self.to_json_string();

        if let Ok(json) = result {
            return write!(f, "{}", json);
        }

        write!(f, "{{}}")
    }
}

fn timestamps_cols() -> impl FormatInto<React> {
    let create_date_column = react::import("typeorm", "CreateDateColumn");
    let update_date_column = react::import("typeorm", "UpdateDateColumn");

    quote_fn! {
        @$create_date_column()
        public created_at: Date;

        @$update_date_column()
        public updated_at: Date;
    }
}
