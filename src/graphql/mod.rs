use crate::database::{self, PrimaryDb};

pub(crate) mod inputs;
pub mod schema;

pub struct Context {
    pub connection: PrimaryDb
}

impl juniper::Context for Context {}

pub struct MutationRoot;

pub struct QueryRoot;


#[derive(GraphQLInputObject)]
pub struct Constraints {
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            limit: 50,
            offset: 0,
        }
    }
}

impl Into<database::Constraints> for Constraints {
    fn into(self) -> database::Constraints {
        database::Constraints {
            limit: self.limit as i64,
            offset: self.offset as i64,
        }
    }
}
