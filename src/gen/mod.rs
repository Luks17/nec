mod database;

use crate::{schema::RootSchema, setup::project::Project};

pub fn start(project: Project, schema: RootSchema) {
    let user_tks = schema.users.user_model();
    project.write_file("src/database/models/Usuario.ts", user_tks, None);
}
