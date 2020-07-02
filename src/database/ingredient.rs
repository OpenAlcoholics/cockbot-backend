use diesel::prelude::*;

use crate::database::{Constraints, DieselResult, GenericIngredient};
use crate::database::schema::ingredient::{self, *};
use crate::models;

#[derive(Debug, Queryable)]
pub struct Ingredient {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) image_link: Option<String>,
    pub(crate) notes: Option<String>,
    pub(crate) alcohol_percentage: i32,
    pub(crate) generic_ingredient_id: i32,
}

impl Ingredient {
    fn from_database_model((ingredient, generic_ingredient): (Ingredient, GenericIngredient)) -> models::Ingredient {
        models::Ingredient {
            id: ingredient.id,
            name: ingredient.name,
            image_link: ingredient.image_link,
            notes: ingredient.notes,
            alcohol_percentage: ingredient.alcohol_percentage,
            generic_ingredient,
        }
    }

    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::Ingredient>> {
        Ok(ingredient::table
            .inner_join(crate::database::schema::generic_ingredient::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load(connection)?
            .into_iter()
            .map(Ingredient::from_database_model)
            .collect())
    }

    pub fn get_by_id(iid: i32, connection: &diesel::PgConnection) -> DieselResult<models::Ingredient> {
        Ok(Ingredient::from_database_model(ingredient::table
            .inner_join(crate::database::schema::generic_ingredient::table)
            .filter(id.eq(iid))
            .load(connection)?
            .pop()
            .ok_or(diesel::NotFound)?))
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `Ingredient` will be simpler though.
    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<Ingredient> {
        diesel::insert_into(table)
            .values(vec![
                (name.eq(self.name), image_link.eq(self.image_link), notes.eq(self.notes), alcohol_percentage.eq(self.alcohol_percentage), generic_ingredient_id.eq(self.generic_ingredient_id))
            ])
            .load::<Ingredient>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
