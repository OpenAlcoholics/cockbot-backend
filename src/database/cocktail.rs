use diesel::prelude::*;

use crate::database::{CocktailAccessory, CocktailIngredient, Constraints, DieselResult, Glass};
use crate::database::cocktail_category::CocktailCategory;
use crate::database::schema::cocktail::{self, *};
use crate::models;

#[derive(Debug, Queryable)]
pub struct Cocktail {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) image_link: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) revision_date: i32,
    pub(crate) notes: Option<String>,
    pub(crate) category_id: i32,
    pub(crate) glass_id: i32,
    pub(crate) ice_cubes: bool,
}

impl Cocktail {
    fn from_database_model((cocktail, category, glass): (Cocktail, CocktailCategory, Glass), connection: &diesel::PgConnection) -> DieselResult<models::Cocktail> {
        let ingredients = CocktailIngredient::get_by_cocktail(cocktail.id, Constraints::default(), connection)?;
        let accessories = CocktailAccessory::get_by_cocktail(cocktail.id, Constraints::default(), connection)?;

        Ok(models::Cocktail {
            id: cocktail.id,
            name: cocktail.name,
            image_link: cocktail.image_link,
            description: cocktail.description,
            revision_date: cocktail.revision_date,
            notes: cocktail.notes,
            category,
            glass,
            ice_cubes: cocktail.ice_cubes,
            ingredients,
            accessories,
        })
    }

    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::Cocktail>> {
        cocktail::table
            .inner_join(crate::database::schema::cocktail_category::table)
            .inner_join(crate::database::schema::glass::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load(connection)?
            .into_iter()
            .map(|model| Cocktail::from_database_model(model, connection))
            .collect()
    }

    pub fn get_by_id(cocktail_id: i32, connection: &diesel::PgConnection) -> DieselResult<models::Cocktail> {
        Cocktail::from_database_model(cocktail::table
                                          .inner_join(crate::database::schema::cocktail_category::table)
                                          .inner_join(crate::database::schema::glass::table)
                                          .filter(id.eq(cocktail_id))
                                          .load(connection)?
                                          .pop()
                                          .ok_or(diesel::NotFound)?, connection)
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `Cocktail` will be simpler though.
    pub fn insert(self, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Cocktail> {
        diesel::insert_into(table)
            .values(vec![
                (name.eq(self.name), image_link.eq(self.image_link), description.eq(self.description), revision_date.eq(self.revision_date), notes.eq(self.notes), category_id.eq(self.category_id), glass_id.eq(self.glass_id), ice_cubes.eq(self.ice_cubes))
            ])
            .load::<Cocktail>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
