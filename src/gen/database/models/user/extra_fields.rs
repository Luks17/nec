use genco::{quote_fn, tokens::FormatInto};

use crate::{
    core::react::{self, React},
    schema::users::extra_field::ExtraFieldSchema,
};

impl ExtraFieldSchema {
    pub fn gen_model_token_function(&self) -> impl FormatInto<React> {
        let column = react::import("typeorm", "Column");
        let name = self.name.to_string();

        quote_fn! {
            @$column({ type: "varchar" })
            $name: string;
        }
    }
}
