use diesel::prelude::*;

use crate::database::{Constraints, DieselResult};
use crate::database::schema::ingredient_category::{self, *};

#[derive(Debug, GraphQLObject, Queryable)]
pub struct IngredientCategory {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) image_link: Option<String>,
    pub(crate) is_alcoholic: bool,
}

impl IngredientCategory {
    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<IngredientCategory>> {
        table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<IngredientCategory>(connection)
    }

    pub fn get_by_id(cid: i32, connection: &diesel::PgConnection) -> DieselResult<IngredientCategory> {
        table
            .filter(id.eq(cid))
            .first(connection)
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `IngredientCategory` will be simpler though.
    pub fn insert(self, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<IngredientCategory> {
        diesel::insert_into(table)
            .values(vec![
                (name.eq(self.name), description.eq(self.description), image_link.eq(self.image_link), is_alcoholic.eq(self.is_alcoholic))
            ])
            .load::<IngredientCategory>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
