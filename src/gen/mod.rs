mod user;

use user::user_model;

use crate::{schema::RootSchema, setup::project::Project};

pub fn start(project: Project, _schema: RootSchema) {
    let user_tks = user_model();
    project.write_file("src/database/models/Usuario.ts", user_tks, None);
}
