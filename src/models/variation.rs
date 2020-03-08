use crate::models::Cocktail;

#[derive(Debug, GraphQLObject)]
pub struct Variation {
    pub id: i32,
    pub cocktail: Cocktail,
    pub description: Option<String>,
}
