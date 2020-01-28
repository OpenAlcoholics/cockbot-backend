use crate::database::cocktail_category::CocktailCategory;
use crate::database::Glass;
use crate::models::CocktailIngredient;

#[derive(Debug, Queryable)]
pub struct Cocktail {
    pub id: i32,
    pub name: String,
    pub image_link: Option<String>,
    pub description: Option<String>,
    pub revision_date: i32,
    pub notes: Option<String>,
    pub category: CocktailCategory,
    pub glass: Glass,
    pub ice_cubes: bool,
    pub ingredients: Vec<CocktailIngredient>
}
