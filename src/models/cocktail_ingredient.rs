use crate::database::IngredientCategory;
use crate::models::Cocktail;

#[derive(Debug, Queryable)]
pub struct CocktailIngredient {
    pub category: IngredientCategory,
    pub share: i32,
    pub rank: Option<i32>,
}
