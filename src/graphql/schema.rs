use juniper::FieldResult;

use crate::database::{self, AccessoryCategory, CocktailCategory, Glass, IngredientCategory};
use crate::graphql::{Constraints, Context, MutationRoot, QueryRoot};
use crate::graphql::inputs::{self, AccessoryCategoryInput, AccessoryInput, CocktailAccessoryInput};
use crate::graphql::queries::AccessoryCategoryQuery;
use crate::models::{Accessory, Cocktail, CocktailAccessory, CocktailIngredient, Ingredient};

#[juniper::object (Context = Context)]
impl MutationRoot {
    fn accessory(&self, context: &Context, input: AccessoryInput) -> FieldResult<Accessory> {
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
    }

    fn accessory_category(&self, context: &Context, input: AccessoryCategoryInput) -> FieldResult<AccessoryCategory> {
        let accessory_category: AccessoryCategory = input.into();
        accessory_category.insert(
            &context.connection.0
        ).map_err(Into::into)
    }

    #[graphql_description = "Returns the ID of the inserted cocktail accessory"]
    fn cocktail_accessory(&self, context: &Context, input: CocktailAccessoryInput) -> FieldResult<CocktailAccessory> {
        let model: database::CocktailAccessory = input.into();
        let cocktail_accessory = model.insert(&context.connection.0)?;

        Ok(CocktailAccessory {
            accessory_category: AccessoryCategory::get_by_id(cocktail_accessory.accessory_category_id, &context.connection.0)?,
            pieces: cocktail_accessory.pieces,
        })
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
