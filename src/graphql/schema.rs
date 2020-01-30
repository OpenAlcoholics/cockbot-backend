use juniper::FieldResult;

use crate::database::{self, AccessoryCategory, CocktailCategory, Glass, IngredientCategory};
use crate::graphql::{Constraints, Context, MutationRoot, QueryRoot};
use crate::graphql::inputs::{self, AccessoryCategoryInput, AccessoryInput, CocktailAccessoryInput, CocktailCategoryInput, CocktailIngredientInput, GlassInput};
use crate::graphql::queries::AccessoryCategoryQuery;
use crate::models::{Accessory, Cocktail, CocktailAccessory, CocktailIngredient, Ingredient};

#[juniper::object (Context = Context)]
impl MutationRoot {
    fn accessories(&self, context: &Context, inputs: Vec<AccessoryInput>) -> FieldResult<Vec<Accessory>> {
        inputs
            .into_iter()
            .map(|input| {
                let accessory: database::Accessory = input.into();
                let accessory = accessory.insert(
                    &context.connection.0
                )?;
                let category = AccessoryCategory::get_by_id(accessory.category_id, &context.connection.0)?;

                Ok(Accessory {
                    id: accessory.id,
                    name: accessory.name,
                    description: accessory.description,
                    image_link: accessory.image_link,
                    category,
                })
            }).collect()
    }

    fn accessory_categories(&self, context: &Context, inputs: Vec<AccessoryCategoryInput>) -> FieldResult<Vec<AccessoryCategory>> {
        inputs
            .into_iter()
            .map(|input| {
                let accessory_category: AccessoryCategory = input.into();
                accessory_category.insert(
                    &context.connection.0
                ).map_err(Into::into)
            }).collect()
    }

    fn cocktail_accessory(&self, context: &Context, inputs: Vec<CocktailAccessoryInput>) -> FieldResult<Vec<CocktailAccessory>> {
        inputs
            .into_iter()
            .map(|input| {
                let model: database::CocktailAccessory = input.into();
                let cocktail_accessory = model.insert(&context.connection.0)?;

                Ok(CocktailAccessory {
                    accessory_category: AccessoryCategory::get_by_id(cocktail_accessory.accessory_category_id, &context.connection.0)?,
                    pieces: cocktail_accessory.pieces,
                })
            }).collect()
    }

    fn cocktail_categories(&self, context: &Context, inputs: Vec<CocktailCategoryInput>) -> FieldResult<Vec<CocktailCategory>> {
        inputs
            .into_iter()
            .map(|input| {
                let model: database::CocktailCategory = input.into();

                model.insert(&context.connection.0).map_err(Into::into)
            }).collect()
    }

    fn cocktail_ingredient(&self, context: &Context, inputs: Vec<CocktailIngredientInput>) -> FieldResult<Vec<CocktailIngredient>> {
        inputs
            .into_iter()
            .map(|input| {
                let model: database::CocktailIngredient = input.into();
                let cocktail_ingredient = model.insert(&context.connection.0)?;

                Ok(CocktailIngredient {
                    category: IngredientCategory::get_by_id(cocktail_ingredient.ingredient_category_id, &context.connection.0)?,
                    share: cocktail_ingredient.share,
                    rank: cocktail_ingredient.rank,
                })
            }).collect()
    }

    fn glass(&self, context: &Context, input: GlassInput) -> FieldResult<Glass> {
        let model: Glass = input.into();

        model.insert(&context.connection.0).map_err(Into::into)
    }
}

#[juniper::object (Context = Context)]
impl QueryRoot {
    fn accessories(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<Accessory>> {
        database::Accessory::get(
            constraints.unwrap_or_default().into(),
            &context.connection.0,
        ).map_err(Into::into)
    }

    fn accessory_categories(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<AccessoryCategory>> {
        AccessoryCategory::get(
            constraints.unwrap_or_default().into(),
            &context.connection.0,
        ).map_err(Into::into)
    }

    fn cocktails(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<Cocktail>> {
        database::Cocktail::get(
            constraints.unwrap_or_default().into(),
            &context.connection.0,
        ).map_err(Into::into)
    }

    fn cocktail_categories(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<CocktailCategory>> {
        database::CocktailCategory::get(
            constraints.unwrap_or_default().into(),
            &context.connection.0,
        ).map_err(Into::into)
    }

    fn glasses(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<Glass>> {
        Glass::get(
            constraints.unwrap_or_default().into(),
            &context.connection.0,
        ).map_err(Into::into)
    }

    fn ingredients(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<Ingredient>> {
        database::Ingredient::get(
            constraints.unwrap_or_default().into(),
            &context.connection.0,
        ).map_err(Into::into)
    }

    fn ingredient_categories(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<IngredientCategory>> {
        IngredientCategory::get(
            constraints.unwrap_or_default().into(),
            &context.connection.0,
        ).map_err(Into::into)
    }
}
