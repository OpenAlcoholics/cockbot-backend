use crate::database::Cocktail;
use crate::graphql::inputs::{CocktailAccessoryInput, CocktailIngredientInput, CocktailTagInput};

#[derive(Debug, GraphQLInputObject)]
pub(crate) struct CocktailIDInput {
    pub name: String,
    pub image_link: Option<String>,
    pub description: Option<String>,
    pub revision_date: i32,
    pub notes: Option<String>,
    pub glass_id: i32,
    pub ice_cubes: bool,
    pub ingredients: Vec<CocktailIngredientInput>,
    pub accessories: Option<Vec<CocktailAccessoryInput>>,
    pub tags: Option<Vec<CocktailTagInput>>,
}

impl Into<Cocktail> for CocktailIDInput {
    fn into(self) -> Cocktail {
        Cocktail {
            id: -1,
            name: self.name,
            image_link: self.image_link,
            description: self.description,
            revision_date: self.revision_date,
            notes: self.notes,
            glass_id: self.glass_id,
            ice_cubes: self.ice_cubes,
        }
    }
}
