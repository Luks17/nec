use genco::{quote_fn, tokens::FormatInto};

use crate::{
    core::react::{self, React},
    gen::database::models::TypeOrmColumnOptions,
    schema::users::extra_field::{ExtraFieldSchema, FieldType},
};

impl FieldType {
    fn type_orm_column_options(&self) -> String {
        let mut options = TypeOrmColumnOptions::new();

        match self {
            FieldType::Cpf => {
                options.type_ = Some("char".to_string());
                options.unique = Some(true);
                options.length = Some(14);
            }
            FieldType::Phone => {
                options.type_ = Some("char".to_string());
                options.unique = Some(true);
                options.length = Some(15);
            }
            FieldType::Email => {
                options.type_ = Some("varchar".to_string());
                options.unique = Some(true);
                options.length = Some(255);
            }
            FieldType::Number => {
                options.type_ = Some("int".to_string());
            }
        };

        options.to_string()
    }
}

impl ExtraFieldSchema {
    pub fn gen_model_token_function(&self) -> impl FormatInto<React> {
        let column = react::import("typeorm", "Column");
        let name = self.name.to_string();
        let options = self.field_type.type_orm_column_options();

        quote_fn! {
            @$column($options)
            $name: string;
        }
    }
}
