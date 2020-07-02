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
    cocktail_tag (tag_id, cocktail_id) {
        tag_id -> Varchar,
        cocktail_id -> Int4,
    }
}

table! {
    generic_ingredient (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        image_link -> Nullable<Varchar>,
        is_alcoholic -> Bool,
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
        generic_ingredient_id -> Int4,
    }
}

table! {
    recipe (cocktail_id, generic_ingredient_id) {
        cocktail_id -> Int4,
        generic_ingredient_id -> Int4,
        share -> Int4,
        rank -> Nullable<Int4>,
    }
}

table! {
    tag (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
    variation (id) {
        id -> Int4,
        cocktail_id -> Int4,
        description -> Nullable<Text>,
    }
}

table! {
    variation_accessory (variation_id, accessory_id) {
        variation_id -> Int4,
        accessory_id -> Int4,
    }
}

table! {
    variation_ingredient (variation_id, ingredient_id) {
        variation_id -> Int4,
        ingredient_id -> Int4,
    }
}

joinable!(accessory -> accessory_category (category_id));
joinable!(cocktail -> glass (glass_id));
joinable!(cocktail_accessory -> accessory_category (accessory_category_id));
joinable!(cocktail_accessory -> cocktail (cocktail_id));
joinable!(cocktail_tag -> cocktail (cocktail_id));
joinable!(cocktail_tag -> tag (tag_id));
joinable!(ingredient -> generic_ingredient (generic_ingredient_id));
joinable!(recipe -> cocktail (cocktail_id));
joinable!(recipe -> generic_ingredient (generic_ingredient_id));
joinable!(variation -> cocktail (cocktail_id));

allow_tables_to_appear_in_same_query!(
    accessory,
    accessory_category,
    cocktail,
    cocktail_accessory,
    cocktail_tag,
    generic_ingredient,
    glass,
    ingredient,
    recipe,
    tag,
    variation,
    variation_accessory,
    variation_ingredient,
);
