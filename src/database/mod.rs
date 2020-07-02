pub use accessory::Accessory;
pub use accessory_category::AccessoryCategory;
pub use cocktail::Cocktail;
pub use cocktail_accessory::CocktailAccessory;
pub use cocktail_ingredient::CocktailIngredient;
pub use cocktail_tag::CocktailTag;
pub use generic_ingredient::GenericIngredient;
pub use glass::Glass;
pub use ingredient::Ingredient;
pub use primary::PrimaryDb;
pub use tag::Tag;
pub use variation::Variation;
pub use variation_accessory::VariationAccessory;
pub use variation_ingredient::VariationIngredient;

pub mod accessory;
pub mod accessory_category;
pub mod cocktail;
pub mod cocktail_tag;
pub mod cocktail_accessory;
pub mod glass;
pub mod ingredient;
pub mod generic_ingredient;
pub mod cocktail_ingredient;
pub mod tag;
pub mod variation;
pub mod variation_ingredient;
pub mod variation_accessory;
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
