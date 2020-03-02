table! {
    accessory (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        image_link -> Nullable<Varchar>,
        category_id -> Int4,
    }
}

table! {
    accessory_category (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        image_link -> Nullable<Varchar>,
    }
}

table! {
    cocktail (id) {
        id -> Int4,
        name -> Varchar,
        image_link -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        revision_date -> Int4,
        notes -> Nullable<Text>,
        glass_id -> Int4,
        ice_cubes -> Bool,
    }
}

table! {
    cocktail_accessory (cocktail_id, accessory_category_id) {
        cocktail_id -> Int4,
        accessory_category_id -> Int4,
        pieces -> Nullable<Int4>,
    }
}

table! {
    cocktail_accessory_suggestion (cocktail_id, accessory_category_id, accessory_id) {
        cocktail_id -> Int4,
        accessory_category_id -> Int4,
        accessory_id -> Int4,
    }
}

table! {
    glass (id) {
        id -> Int4,
        name -> Varchar,
        estimated_size -> Nullable<Int4>,
        image_link -> Nullable<Varchar>,
    }
}

table! {
    ingredient (id) {
        id -> Int4,
        name -> Varchar,
        image_link -> Nullable<Varchar>,
        notes -> Nullable<Varchar>,
        alcohol_percentage -> Int4,
        category_id -> Int4,
    }
}

table! {
    ingredient_category (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        image_link -> Nullable<Varchar>,
        is_alcoholic -> Bool,
    }
}

table! {
    recipe (cocktail_id, ingredient_category_id) {
        cocktail_id -> Int4,
        ingredient_category_id -> Int4,
        share -> Int4,
        rank -> Nullable<Int4>,
    }
}

table! {
    recipe_ingredient_suggestion (cocktail_id, ingredient_category_id, ingredient_id) {
        cocktail_id -> Int4,
        ingredient_category_id -> Int4,
        ingredient_id -> Int4,
    }
}

joinable!(accessory -> accessory_category (category_id));
joinable!(cocktail -> glass (glass_id));
joinable!(cocktail_accessory -> accessory_category (accessory_category_id));
joinable!(cocktail_accessory -> cocktail (cocktail_id));
joinable!(ingredient -> ingredient_category (category_id));
joinable!(recipe -> cocktail (cocktail_id));
joinable!(recipe -> ingredient_category (ingredient_category_id));

allow_tables_to_appear_in_same_query!(
    accessory,
    accessory_category,
    cocktail,
    cocktail_accessory,
    cocktail_accessory_suggestion,
    glass,
    ingredient,
    ingredient_category,
    recipe,
    recipe_ingredient_suggestion,
);
