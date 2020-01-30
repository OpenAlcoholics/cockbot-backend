use diesel::prelude::*;

use crate::database::{Cocktail, Constraints, DieselResult, IngredientCategory};
use crate::database::schema::recipe::{self, *};
use crate::models;

#[derive(Debug, Queryable)]
pub struct CocktailIngredient {
    // id: i32,
    pub(crate) cocktail_id: i32,
    pub(crate) ingredient_category_id: i32,
    pub(crate) share: i32,
    pub(crate) rank: Option<i32>,
}

impl CocktailIngredient {
    fn from_database_model((recipe, ingredient_category): (CocktailIngredient, IngredientCategory), connection: &diesel::PgConnection) -> DieselResult<models::CocktailIngredient> {
        Ok(models::CocktailIngredient {
            category: ingredient_category,
            share: recipe.share,
            rank: recipe.rank,
        })
    }

    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::CocktailIngredient>> {
        recipe::table
            .inner_join(crate::database::schema::ingredient_category::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load(connection)?
            .into_iter()
            .map(|x| CocktailIngredient::from_database_model(x, connection))
            .collect()
    }

    pub fn get_by_cocktail(cid: i32, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::CocktailIngredient>> {
        recipe::table
            .inner_join(crate::database::schema::ingredient_category::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .filter(cocktail_id.eq(cid))
            .load(connection)?
            .into_iter()
            .map(|x| CocktailIngredient::from_database_model(x, connection))
            .collect()
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `Recipe` will be simpler though.
    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<CocktailIngredient> {
        diesel::insert_into(table)
            .values(vec![
                (cocktail_id.eq(self.cocktail_id), ingredient_category_id.eq(self.ingredient_category_id), share.eq(self.share), rank.eq(self.rank))
            ])
            .load::<CocktailIngredient>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
