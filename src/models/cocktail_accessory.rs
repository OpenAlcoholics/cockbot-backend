use crate::database::AccessoryCategory;
use crate::models::Cocktail;

#[derive(Debug, GraphQLObject)]
pub struct CocktailAccessory {
    pub(crate) accessory_category: AccessoryCategory,
    pub(crate) pieces: Option<i32>,
}
