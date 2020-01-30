use diesel::prelude::*;

use crate::database::{Constraints, DieselResult};
use crate::database::schema::glass::{self, *};

#[derive(Debug, GraphQLObject, Queryable)]
pub struct Glass {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) estimated_size: Option<i32>,
    pub(crate) image_link: Option<String>,
}

impl Glass {
    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<Glass>> {
        table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<Glass>(connection)
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `Accessory` will be simpler though.
    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<Glass> {
        diesel::insert_into(table)
            .values(vec![
                (name.eq(self.name), estimated_size.eq(self.estimated_size), image_link.eq(self.image_link))
            ])
            .load::<Glass>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
