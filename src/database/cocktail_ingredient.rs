use diesel::prelude::*;

use crate::database::{Cocktail, Constraints, DieselResult, GenericIngredient};
use crate::database::schema::recipe::{self, *};
use crate::models;

#[derive(Clone, Debug, Queryable)]
pub struct CocktailIngredient {
    // id: i32,
    pub(crate) cocktail_id: i32,
    pub(crate) generic_ingredient_id: i32,
    pub(crate) share: i32,
    pub(crate) rank: Option<i32>,
}

impl CocktailIngredient {
    fn from_database_model((recipe, generic_ingredient): (CocktailIngredient, GenericIngredient)) -> DieselResult<models::CocktailIngredient> {
        Ok(models::CocktailIngredient {
            generic_ingredient,
            share: recipe.share,
            rank: recipe.rank,
        })
    }

    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::CocktailIngredient>> {
        recipe::table
            .inner_join(crate::database::schema::generic_ingredient::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load(connection)?
            .into_iter()
            .map(|x| CocktailIngredient::from_database_model(x))
            .collect()
    }

    pub fn get_by_cocktail(cid: i32, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::CocktailIngredient>> {
        recipe::table
            .inner_join(crate::database::schema::generic_ingredient::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .filter(cocktail_id.eq(cid))
            .load(connection)?
            .into_iter()
            .map(|x| CocktailIngredient::from_database_model(x))
            .collect()
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `Recipe` will be simpler though.
    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<CocktailIngredient> {
        diesel::insert_into(table)
            .values(vec![
                (cocktail_id.eq(self.cocktail_id), generic_ingredient_id.eq(self.generic_ingredient_id), share.eq(self.share), rank.eq(self.rank))
            ])
            .load::<CocktailIngredient>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
