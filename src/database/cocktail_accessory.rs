use diesel::prelude::*;

use crate::database::{Constraints, DieselResult};
use crate::database::schema::cocktail_accessory::{self, *};

#[derive(Debug, Queryable)]
pub struct CocktailAccessory {
    cocktail_id: i32,
    accessory_category_id: i32,
    pieces: Option<i32>,
}

impl CocktailAccessory {
    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<CocktailAccessory>> {
        table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<CocktailAccessory>(connection)
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `CocktailAccessory` will be simpler though.
    pub fn insert(self, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<CocktailAccessory> {
        diesel::insert_into(table)
            .values(vec![
                (cocktail_id.eq(self.cocktail_id), accessory_category_id.eq(self.accessory_category_id), pieces.eq(self.pieces))
            ])
            .load::<CocktailAccessory>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
