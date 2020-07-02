use diesel::prelude::*;

use crate::database::{Cocktail, Constraints, DieselResult, GenericIngredient, Ingredient, Variation};
use crate::database::schema::variation_ingredient::{self, *};
use crate::models;

#[derive(Clone, Debug, Queryable)]
pub struct VariationIngredient {
    pub(crate) variation_id: i32,
    pub(crate) ingredient_id: i32,
}

impl VariationIngredient {
    fn from_database_model(variation_ingredient: VariationIngredient, connection: &diesel::PgConnection) -> DieselResult<models::VariationIngredient> {
        let variation = Variation::get_by_id(variation_ingredient.variation_id, connection)?;
        let ingredient = Ingredient::get_by_id(variation_ingredient.ingredient_id, connection)?;

        Ok(models::VariationIngredient {
            variation,
            ingredient,
        })
    }

    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::VariationIngredient>> {
        variation_ingredient::table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load(connection)?
            .into_iter()
            .map(|x| VariationIngredient::from_database_model(x, connection))
            .collect()
    }

    pub fn get_by_variation(vid: i32, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::VariationIngredient>> {
        variation_ingredient::table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .filter(variation_id.eq(vid))
            .load(connection)?
            .into_iter()
            .map(|x| VariationIngredient::from_database_model(x, connection))
            .collect()
    }

    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<VariationIngredient> {
        diesel::insert_into(table)
            .values(vec![
                (variation_id.eq(self.variation_id), ingredient_id.eq(self.ingredient_id))
            ])
            .load::<VariationIngredient>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
