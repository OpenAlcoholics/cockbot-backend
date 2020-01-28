use juniper::FieldResult;

use crate::database::AccessoryCategory;
use crate::graphql::{Constraints, Context, MutationRoot, QueryRoot};

#[juniper::object (Context = Context)]
impl MutationRoot {}

#[juniper::object (Context = Context)]
impl QueryRoot {
    fn accessory_categories(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<AccessoryCategory>> {
        AccessoryCategory::get(
            constraints.unwrap_or_default().into(),
            &context.connection.0,
        ).map_err(Into::into)
    }
}
