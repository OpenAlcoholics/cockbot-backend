use crate::models::Cocktail;
use crate::database::IngredientCategory;

#[derive(Debug, Queryable)]
pub struct CocktailIngredient {
    pub ingredient_category: IngredientCategory,
    pub share: i32,
    pub rank: Option<i32>,
}
