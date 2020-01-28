use crate::database::AccessoryCategory;
use crate::models::Cocktail;

#[derive(Debug, GraphQLObject)]
pub struct CocktailAccessory {
    cocktail: Cocktail,
    accessory_category: AccessoryCategory,
    pieces: Option<i32>,
}
