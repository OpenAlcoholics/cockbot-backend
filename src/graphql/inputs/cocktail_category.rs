use crate::database;
use crate::database::CocktailCategory;

#[derive(Debug, GraphQLInputObject)]
pub struct CocktailCategoryInput {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) image_link: Option<String>,
}

impl Into<database::CocktailCategory> for CocktailCategoryInput {
    fn into(self) -> database::CocktailCategory {
        database::CocktailCategory {
            id: -1,
            name: self.name,
            description: self.description,
            image_link: self.image_link,
        }
    }
}
