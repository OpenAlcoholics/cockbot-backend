use crate::models::Cocktail;
use crate::database::AccessoryCategory;

pub struct CocktailAccessory {
    cocktail: Cocktail,
    accessory_category: AccessoryCategory,
    pieces: Option<i32>
}
