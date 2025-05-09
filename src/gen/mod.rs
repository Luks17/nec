mod database;

use crate::{schema::RootSchema, setup::project::Project};

pub struct Generator {
    project: Project,
    schema: RootSchema,
}

impl Generator {
    pub fn new(project: Project, schema: RootSchema) -> Self {
        Self { project, schema }
    }

    pub fn generate(&self) {
        self.generate_database();
    }
}
