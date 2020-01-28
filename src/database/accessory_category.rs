use diesel::prelude::*;

use crate::database::{Constraints, DieselResult};
use crate::database::schema::accessory_category as ac;

#[derive(Debug, GraphQLObject, Queryable)]
pub struct AccessoryCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_link: Option<String>,
}

impl AccessoryCategory {
    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<AccessoryCategory>> {
        ac::table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<AccessoryCategory>(connection)
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `AccessoryCategory` will be simpler though.
    pub fn insert(self, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<AccessoryCategory> {
        use ac::columns::*;

        diesel::insert_into(ac::table)
            .values(vec![
                (name.eq(self.name), description.eq(self.description), image_link.eq(self.image_link))
            ])
            .load::<AccessoryCategory>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
