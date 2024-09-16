use genco::{quote, Tokens};

use crate::core::react::{self, React};

pub fn user_model() -> Tokens<React> {
    let primary_column = &react::import("typeorm", "PrimaryColumn");
    let entity = &react::import("typeorm", "Entity");
    let column = &react::import("typeorm", "Column");

    quote! {
        @$entity("usuarios")
        export class Usuario {
            @$primary_column({ type: "char", length: 36 })
            id: string;

            @$column({ type: "varchar", length: 256 })
            nome: string;
        }
    }
}
