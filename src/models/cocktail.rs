use crate::database::Glass;
use crate::models::{CocktailAccessory, CocktailIngredient};

#[derive(Debug, GraphQLObject)]
pub struct Cocktail {
    pub id: i32,
    pub name: String,
    pub image_link: Option<String>,
    pub description: Option<String>,
    pub revision_date: i32,
    pub notes: Option<String>,
    pub glass: Glass,
    pub ice_cubes: bool,
    pub ingredients: Vec<CocktailIngredient>,
    pub accessories: Vec<CocktailAccessory>,
}
