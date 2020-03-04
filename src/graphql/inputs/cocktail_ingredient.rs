use crate::database;

#[derive(Debug, GraphQLInputObject)]
pub struct CocktailIngredientInput {
    cocktail_id: i32,
    ingredient_category_id: i32,
    share: i32,
    rank: Option<i32>,
}

impl<'a> Into<database::CocktailIngredient> for &'a CocktailIngredientInput {
    fn into(self) -> database::CocktailIngredient {
        database::CocktailIngredient {
            cocktail_id: self.cocktail_id,
            ingredient_category_id: self.ingredient_category_id,
            share: self.share,
            rank: self.rank,
        }
    }
}

impl Into<database::CocktailIngredient> for CocktailIngredientInput {
    fn into(self) -> database::CocktailIngredient {
        database::CocktailIngredient {
            cocktail_id: self.cocktail_id,
            ingredient_category_id: self.ingredient_category_id,
            share: self.share,
            rank: self.rank,
        }
    }
}
