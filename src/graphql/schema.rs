use juniper::FieldResult;

use crate::database::{self, AccessoryCategory, CocktailTag, GenericIngredient, Glass, Tag};
use crate::graphql::{Constraints, Context, MutationRoot, QueryRoot};
use crate::graphql::inputs::{self, AccessoryCategoryInput, AccessoryInput, CocktailAccessoryInput, CocktailIDInput, CocktailIngredientInput, CocktailTagInput, GlassInput, TagInput};
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

    fn cocktail(&self, context: &Context, inputs: Vec<CocktailIDInput>) -> FieldResult<Vec<Cocktail>> {
        inputs
            .into_iter()
            .map(|input| {
                input
                    .ingredients
                    .iter()
                    .map(|input| {
                        let model: database::CocktailIngredient = input.into();
                        model
                            .insert(&context.connection.0)
                            .map_err(Into::into)
                    }).collect::<FieldResult<Vec<database::CocktailIngredient>>>()?;
                input
                    .accessories
                    .unwrap_or(vec![])
                    .iter()
                    .map(|input| {
                        let model: database::CocktailAccessory = input.into();
                        model
                            .insert(&context.connection.0)
                            .map_err(Into::into)
                    }).collect::<FieldResult<Vec<database::CocktailAccessory>>>()?;
                input
                    .tags
                    .unwrap_or(vec![])
                    .iter()
                    .map(|input| {
                        let model: database::CocktailTag = input.into();
                        model
                            .insert(&context.connection.0)
                            .map_err(Into::into)
                    }).collect::<FieldResult<Vec<database::CocktailTag>>>()?;

                let cocktail = database::Cocktail {
                    id: -1,
                    name: input.name,
                    image_link: input.image_link,
                    description: input.description,
                    revision_date: input.revision_date,
                    notes: input.notes,
                    glass_id: input.glass_id,
                    ice_cubes: input.ice_cubes,
                };

                let id = cocktail
                    .insert(&context.connection.0)?
                    .id;

                database::Cocktail::get_by_id(id, &context.connection.0)
                    .map_err(Into::into)
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

    fn cocktail_ingredients(&self, context: &Context, inputs: Vec<CocktailIngredientInput>) -> FieldResult<Vec<CocktailIngredient>> {
        inputs
            .into_iter()
            .map(|input| {
                let model: database::CocktailIngredient = input.into();
                let cocktail_ingredient = model.insert(&context.connection.0)?;

                Ok(CocktailIngredient {
                    generic_ingredient: GenericIngredient::get_by_id(cocktail_ingredient.generic_ingredient_id, &context.connection.0)?,
                    share: cocktail_ingredient.share,
                    rank: cocktail_ingredient.rank,
                })
            }).collect()
    }

    fn cocktail_tag(&self, context: &Context, inputs: Vec<CocktailTagInput>) -> FieldResult<Vec<CocktailTag>> {
        inputs
            .into_iter()
            .map(|input: CocktailTagInput| {
                let tag: CocktailTag = input.into();

                tag
                    .insert(&context.connection.0)
                    .map_err(Into::into)
            }).collect()
    }

    fn glasses(&self, context: &Context, inputs: Vec<GlassInput>) -> FieldResult<Vec<Glass>> {
        inputs
            .into_iter()
            .map(|input| {
                let model: Glass = input.into();

                model.insert(&context.connection.0).map_err(Into::into)
            }).collect()
    }

    fn tags(&self, context: &Context, inputs: Vec<TagInput>) -> FieldResult<Vec<Tag>> {
        inputs
            .into_iter()
            .map(|input| {
                let model: Tag = input.into();

                model.insert(&context.connection.0).map_err(Into::into)
            }).collect()
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

    fn glasses(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<Glass>> {
        Glass::get(
            constraints.unwrap_or_default().into(),
            &context.connection.0,
        ).map_err(Into::into)
    }

    fn tags(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<Tag>> {
        Tag::get(
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

    fn ingredient_categories(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<GenericIngredient>> {
        GenericIngredient::get(
            constraints.unwrap_or_default().into(),
            &context.connection.0,
        ).map_err(Into::into)
    }
}
