use crate::database;

#[derive(Debug, GraphQLInputObject)]
pub struct CocktailAccessoryInput {
    pub cocktail_id: i32,
    pub accessory_category_id: i32,
    pub pieces: Option<i32>,
}

impl Into<database::CocktailAccessory> for CocktailAccessoryInput {
    fn into(self) -> database::CocktailAccessory {
        database::CocktailAccessory {
            cocktail_id: self.cocktail_id,
            accessory_category_id: self.accessory_category_id,
            pieces: self.pieces,
        }
    }
}
