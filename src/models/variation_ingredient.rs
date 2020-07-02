use crate::models::{Ingredient, Variation};

#[derive(Debug, GraphQLObject)]
pub struct VariationIngredient {
    pub variation: Variation,
    pub ingredient: Ingredient,
}
