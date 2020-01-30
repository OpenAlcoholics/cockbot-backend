use diesel::prelude::*;

use crate::database::{Constraints, DieselResult};
use crate::database::schema::cocktail_category::{self, *};

#[derive(Debug, GraphQLObject, Queryable)]
pub struct CocktailCategory {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) image_link: Option<String>,
}

impl CocktailCategory {
    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<CocktailCategory>> {
        table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<CocktailCategory>(connection)
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `CocktailCategory` will be simpler though.
    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<CocktailCategory> {
        diesel::insert_into(table)
            .values(vec![
                (name.eq(self.name), description.eq(self.description), image_link.eq(self.image_link))
            ])
            .load::<CocktailCategory>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
