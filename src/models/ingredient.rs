use crate::database::IngredientCategory;

#[derive(Debug, GraphQLObject)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub image_link: Option<String>,
    pub notes: Option<String>,
    pub alcohol_percentage: i32,
    pub category: IngredientCategory,
}
