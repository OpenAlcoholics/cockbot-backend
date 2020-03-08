use crate::database::GenericIngredient;
use crate::models::Cocktail;

#[derive(Debug, GraphQLObject)]
pub struct CocktailIngredient {
    pub generic_ingredient: GenericIngredient,
    pub share: i32,
    pub rank: Option<i32>,
}
