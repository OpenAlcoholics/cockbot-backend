use crate::database::IngredientCategory;
use crate::models::Cocktail;

#[derive(Debug, GraphQLObject)]
pub struct CocktailIngredient {
    pub category: IngredientCategory,
    pub share: i32,
    pub rank: Option<i32>,
}
