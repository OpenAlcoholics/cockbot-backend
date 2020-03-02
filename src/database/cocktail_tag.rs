use diesel::prelude::*;

use crate::database::{Cocktail, Constraints, DieselResult, Tag};
use crate::database::schema::cocktail_tag::{self, *};
use crate::models;

#[derive(Debug, GraphQLObject, Queryable)]
pub struct CocktailTag {
    pub(crate) tag_id: String,
    pub(crate) cocktail_id: i32,
}

impl CocktailTag {
    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `Recipe` will be simpler though.
    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<CocktailTag> {
        diesel::insert_into(table)
            .values(vec![
                (tag_id.eq(self.tag_id), cocktail_id.eq(self.cocktail_id))
            ])
            .load::<CocktailTag>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
