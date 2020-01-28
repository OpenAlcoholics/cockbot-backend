use diesel::prelude::*;

use crate::database::{AccessoryCategory, Cocktail, Constraints, DieselResult};
use crate::database::schema::cocktail_accessory::{self, *};
use crate::models;

#[derive(Debug, Queryable)]
pub struct CocktailAccessory {
    cocktail_id: i32,
    accessory_category_id: i32,
    pieces: Option<i32>,
}

impl CocktailAccessory {
    fn from_database_model((cocktail_accessory, accessory_category): (CocktailAccessory, AccessoryCategory), connection: &diesel::PgConnection) -> DieselResult<models::CocktailAccessory> {
        Ok(models::CocktailAccessory {
            accessory_category,
            pieces: cocktail_accessory.pieces,
        })
    }

    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::CocktailAccessory>> {
        cocktail_accessory::table
            .inner_join(crate::database::schema::accessory_category::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load(connection)?
            .into_iter()
            .map(|x| CocktailAccessory::from_database_model(x, connection))
            .collect()
    }

    pub fn get_by_cocktail(cid: i32, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::CocktailAccessory>> {
        cocktail_accessory::table
            .inner_join(crate::database::schema::accessory_category::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .filter(cocktail_id.eq(cid))
            .load(connection)?
            .into_iter()
            .map(|x| CocktailAccessory::from_database_model(x, connection))
            .collect()
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
