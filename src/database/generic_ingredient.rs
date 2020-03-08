use diesel::prelude::*;

use crate::database::{Constraints, DieselResult};
use crate::database::schema::generic_ingredient::{self, *};

#[derive(Debug, GraphQLObject, Queryable)]
pub struct GenericIngredient {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) image_link: Option<String>,
    pub(crate) is_alcoholic: bool,
}

impl GenericIngredient {
    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<GenericIngredient>> {
        table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<GenericIngredient>(connection)
    }

    pub fn get_by_id(cid: i32, connection: &diesel::PgConnection) -> DieselResult<GenericIngredient> {
        table
            .filter(id.eq(cid))
            .first(connection)
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `GenericIngredient` will be simpler though.
    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<GenericIngredient> {
        diesel::insert_into(table)
            .values(vec![
                (name.eq(self.name), description.eq(self.description), image_link.eq(self.image_link), is_alcoholic.eq(self.is_alcoholic))
            ])
            .load::<GenericIngredient>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
