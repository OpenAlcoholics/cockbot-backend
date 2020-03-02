use diesel::prelude::*;
use regex::Regex;

use crate::database::{CocktailTag, Constraints, DieselResult};
use crate::database::schema::tag::{self, *};

#[derive(Debug, GraphQLObject, Queryable)]
pub struct Tag {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

lazy_static!(
    static ref NORMALIZE_REGEX: Regex = Regex::new("[^a-z]").unwrap();
);

impl Tag {
    pub fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<Tag>> {
        table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<Tag>(connection)
    }

    pub fn get_by_cocktail(cid: i32, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<Tag>> {
        Ok(table
            .inner_join(crate::database::schema::cocktail_tag::table)
            .filter(crate::database::schema::cocktail_tag::cocktail_id.eq(cid))
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<(Tag, CocktailTag)>(connection)?
            .into_iter()
            .map(|(tag, _)| tag)
            .collect())
    }

    // This poses a little bit more work than defining a second struct which derives from `Insertable`, the rest of the code which uses `Tag` will be simpler though.
    pub fn insert(self, connection: &diesel::PgConnection) -> DieselResult<Tag> {
        let normalized_id = Tag::normalize_name(&self.name);

        diesel::insert_into(table)
            .values(vec![
                (id.eq(normalized_id), name.eq(self.name), description.eq(&self.description))
            ])
            .load::<Tag>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }

    fn normalize_name(raw_name: &str) -> String {
        NORMALIZE_REGEX
            .replace_all(&raw_name.to_lowercase(), "")
            .to_string()
    }

    pub fn find_normalized(search: String, connection: &diesel::PgConnection) -> DieselResult<Vec<Tag>> {
        table
            .filter(id.eq(search))
            .load::<Tag>(connection)
    }
}
