pub use accessory::Accessory;
pub use accessory_category::AccessoryCategory;
pub use cocktail::Cocktail;
pub use cocktail_accessory::CocktailAccessory;
pub use cocktail_category::CocktailCategory;
pub use glass::Glass;
pub use ingredient::Ingredient;
pub use ingredient_category::IngredientCategory;
pub use cocktail_ingredient::CocktailIngredient;

pub mod accessory;
pub mod accessory_category;
pub mod cocktail;
pub mod cocktail_accessory;
pub mod cocktail_category;
pub mod glass;
pub mod ingredient;
pub mod ingredient_category;
pub mod cocktail_ingredient;
pub(crate) mod schema;

pub struct Constraints {
    limit: i64,
    offset: i64,
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
