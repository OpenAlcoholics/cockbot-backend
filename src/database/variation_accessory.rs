use diesel::prelude::*;

use crate::database::{Accessory, Cocktail, Constraints, DieselResult, Variation};
use crate::database::schema::variation_accessory::{self, *};
use crate::models;

#[derive(Clone, Debug, Queryable)]
pub struct VariationAccessory {
    pub(crate) variation_id: i32,
    pub(crate) accessory_id: i32,
}

impl VariationAccessory {
    fn from_database_model(variation_accessory: VariationAccessory, connection: &diesel::PgConnection) -> DieselResult<models::VariationAccessory> {
        let variation = Variation::get_by_id(variation_accessory.variation_id, connection)?;
        let accessory = Accessory::get_by_id(variation_accessory.accessory_id, connection)?;

        Ok(models::VariationAccessory {
            variation,
            accessory,
        })
    }

    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::VariationAccessory>> {
        variation_accessory::table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load(connection)?
            .into_iter()
            .map(|x| VariationAccessory::from_database_model(x, connection))
            .collect()
    }

    pub fn get_by_variation(vid: i32, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::VariationAccessory>> {
        variation_accessory::table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .filter(variation_id.eq(vid))
            .load(connection)?
            .into_iter()
            .map(|x| VariationAccessory::from_database_model(x, connection))
            .collect()
    }

    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<VariationAccessory> {
        diesel::insert_into(table)
            .values(vec![
                (variation_id.eq(self.variation_id), accessory_id.eq(self.accessory_id))
            ])
            .load::<VariationAccessory>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
