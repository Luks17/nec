mod extra_fields;

use crate::{
    core::react::{self, React},
    gen::database::timestamps_cols,
    schema::users::UsersSchema,
};
use genco::{quote, Tokens};

impl UsersSchema {
    pub fn user_model(&self) -> Tokens<React> {
        let primary_column = &react::import("typeorm", "PrimaryColumn");
        let entity = &react::import("typeorm", "Entity");
        let column = &react::import("typeorm", "Column");
        let min_length = &react::import("typeorm", "MinLength");

        quote! {
            @$entity("usuarios")
            export class Usuario {
                @$primary_column({ type: "char", length: 36 })
                id: string;

                @$column({ type: "varchar", length: 255 })
                @$min_length(2)
                nome: string;

                @$column({ type: "varchar", length: 255 })
                @$min_length(6)
                password: string;

                $(timestamps_cols());
            }
        }
    }
}
