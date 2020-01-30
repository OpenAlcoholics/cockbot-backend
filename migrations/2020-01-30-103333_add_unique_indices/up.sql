BEGIN TRANSACTION;

CREATE UNIQUE INDEX ingredient_category_unique_name_index ON ingredient_category (UPPER(name));
CREATE UNIQUE INDEX glass_unique_name_esize_index ON glass (estimated_size, UPPER(name));
CREATE UNIQUE INDEX cocktail_category_unique_name_index ON cocktail_category (UPPER(name));
CREATE UNIQUE INDEX cocktail_unique_name_index ON cocktail (UPPER(name), category_id, glass_id); -- is this reasonable? variations of one cocktail should be defined in suggestions
CREATE UNIQUE INDEX ingredient_unique_name_cid_ap_index ON ingredient (UPPER(name), category_id, alcohol_percentage);
CREATE UNIQUE INDEX recipe_unique_index ON recipe (cocktail_id, ingredient_category_id, share, rank);
CREATE UNIQUE INDEX accessory_category_unique_name_index ON accessory_category (UPPER(name));
CREATE UNIQUE INDEX accessory_unique_name_category_index ON accessory (category_id, UPPER(name));
CREATE UNIQUE INDEX recipe_ingredient_suggestion_unique_index ON recipe_ingredient_suggestion (cocktail_id, ingredient_category_id, ingredient_id);
CREATE UNIQUE INDEX recipe_accessory_suggestion_unique_index ON cocktail_accessory_suggestion (cocktail_id, accessory_category_id, accessory_id);

END;
