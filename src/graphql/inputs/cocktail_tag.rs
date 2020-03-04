use crate::database::CocktailTag;

#[derive(Debug, GraphQLInputObject)]
pub struct CocktailTagInput {
    pub tag_id: String,
    pub cocktail_id: i32,
}

impl Into<CocktailTag> for CocktailTagInput {
    fn into(self) -> CocktailTag {
        CocktailTag {
            tag_id: self.tag_id,
            cocktail_id: self.cocktail_id,
        }
    }
}

impl<'a> Into<CocktailTag> for &'a CocktailTagInput {
    fn into(self) -> CocktailTag {
        CocktailTag {
            tag_id: self.tag_id.clone(),
            cocktail_id: self.cocktail_id,
        }
    }
}
