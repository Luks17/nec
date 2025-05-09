use crate::{
    core::{
        react::{self, React},
        utils::snake_case_to_camel_case,
    },
    gen::{database::timestamps_cols, Generator},
    schema::entity::{
        attributes::{AttributeSchema, AttributeType},
        EntitySchema,
    },
};
use genco::{
    quote, quote_fn, quote_in,
    tokens::{from_fn, FormatInto},
    Tokens,
};

impl Generator {
    pub(super) fn generate_entities(&self) {
        for entity in self.schema.entities.iter() {
            let tokens = create_entity_tokens(entity);

            self.project.write_file(
                &format!("src/database/models/{}.ts", entity.name),
                tokens,
                None,
            );
        }
    }
}

fn create_entity_tokens(schema: &EntitySchema) -> Tokens<React> {
    let entity = react::import("typeorm", "Entity");

    quote! {
        @$entity()
        export class $(&schema.name) {
            $(for attr in &schema.attributes join ($['\n']) => $(attr.get_tokens(schema.name.clone())))

            $(if schema.timestamps => $(timestamps_cols()) )
        }
    }
}

impl AttributeSchema {
    fn get_tokens(&self, entity_name: String) -> impl FormatInto<React> + '_ {
        quote_fn! {
            $(&self.name): $(self.type_.get_ts_type_token(entity_name, self.name.clone()));
        }
    }
}

impl AttributeType {
    fn get_ts_type_token(
        &self,
        entity_name: String,
        attr_name: String,
    ) -> impl FormatInto<React> + '_ {
        quote_fn! {
            $(match self {
                AttributeType::String => string,
                AttributeType::Text => string,
                AttributeType::Integer => number,
                AttributeType::Decimal => number,
                AttributeType::Boolean => boolean,
                AttributeType::Date => Date,
                AttributeType::DateTime => Date,
                AttributeType::Enum => $(from_fn(move |t| {
                    let enum_name = snake_case_to_camel_case(format!("{}_{}", entity_name, attr_name));
                    let enum_import = &react::import(format!("@/lib/enums/{}", entity_name), enum_name);

                    quote_in! { *t => $enum_import }
                })),
            })
        }
    }
}
