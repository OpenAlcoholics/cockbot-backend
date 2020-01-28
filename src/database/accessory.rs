use diesel::prelude::*;

use crate::database::{AccessoryCategory, Constraints, DieselResult};
use crate::database::schema::accessory::{self, *};
use crate::models;

#[derive(Debug, Queryable)]
pub struct Accessory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_link: Option<String>,
    pub category_id: i32,
}

impl Accessory {
    fn from_database_model((accessory, category): (Accessory, AccessoryCategory)) -> models::Accessory {
        models::Accessory {
            id: accessory.id,
            name: accessory.name,
            description: accessory.description,
            image_link: accessory.image_link,
            category,
        }
    }

    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::Accessory>> {
        Ok(accessory::table
            .inner_join(crate::database::schema::accessory_category::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load(connection)?
            .into_iter()
            .map(Accessory::from_database_model)
            .collect())
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `Accessory` will be simpler though.
    pub fn insert(self, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Accessory> {
        diesel::insert_into(table)
            .values(vec![
                (name.eq(self.name), description.eq(self.description), image_link.eq(self.image_link), category_id.eq(self.category_id))
            ])
            .load::<Accessory>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
