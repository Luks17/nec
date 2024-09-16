use genco::{quote_fn, tokens::FormatInto};

use crate::core::react::{self, React};

pub mod models;

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