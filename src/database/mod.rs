pub use accessory::Accessory;
pub use accessory_category::AccessoryCategory;
pub use cocktail::Cocktail;
pub use cocktail_accessory::CocktailAccessory;
pub use cocktail_ingredient::CocktailIngredient;
pub use glass::Glass;
pub use ingredient::Ingredient;
pub use ingredient_category::IngredientCategory;
pub use primary::PrimaryDb;

pub mod accessory;
pub mod accessory_category;
pub mod cocktail;
pub mod cocktail_accessory;
pub mod glass;
pub mod ingredient;
pub mod ingredient_category;
pub mod cocktail_ingredient;
pub(crate) mod schema;
mod primary;

pub struct Constraints {
    pub(crate) limit: i64,
    pub(crate) offset: i64,
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            limit: 50,
            offset: 0,
        }
    }
}

type DieselResult<V> = Result<V, diesel::result::Error>;
