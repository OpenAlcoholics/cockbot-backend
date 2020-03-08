use diesel::prelude::*;
use regex::Regex;

use crate::database::{Cocktail, Constraints, DieselResult};
use crate::database::schema::variation::{self, *};
use crate::models;

#[derive(Debug, GraphQLObject, Queryable)]
pub struct Variation {
    pub(crate) id: i32,
    pub(crate) cocktail_id: i32,
    pub(crate) description: Option<String>,
}

impl Variation {
    fn from_database_model((variation, cocktail): (Variation, Cocktail), connection: &diesel::PgConnection) -> DieselResult<models::Variation> {
        let cocktail = Cocktail::get_by_id(cocktail.id, connection)?;

        return Ok(models::Variation {
            id: variation.id,
            cocktail,
            description: variation.description,
        });
    }

    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::Variation>> {
        table
            .inner_join(crate::database::schema::cocktail::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<(Variation, Cocktail)>(connection)?
            .into_iter()
            .map(|x| Variation::from_database_model(x, connection))
            .collect()
    }

    pub fn get_by_cocktail(cid: i32, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<models::Variation> {
        Variation::from_database_model(
            table
                .inner_join(crate::database::schema::cocktail::table)
                .limit(constraints.limit)
                .offset(constraints.offset)
                .filter(cocktail_id.eq(cid))
                .load::<(Variation, Cocktail)>(connection)?
                .pop()
                .ok_or(diesel::NotFound)?,
            connection)
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `Variation` will be simpler though.
    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<Variation> {
        diesel::insert_into(table)
            .values(vec![
                (id.eq(self.id), cocktail_id.eq(self.cocktail_id), description.eq(&self.description))
            ])
            .load::<Variation>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
