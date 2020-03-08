BEGIN TRANSACTION;

CREATE UNIQUE INDEX generic_ingredient_unique_name_index ON generic_ingredient (UPPER(name));
CREATE UNIQUE INDEX glass_unique_name_esize_index ON glass (estimated_size, UPPER(name));
CREATE UNIQUE INDEX ingredient_unique_name_cid_ap_index ON ingredient (UPPER(name), generic_ingredient_id, alcohol_percentage);
CREATE UNIQUE INDEX recipe_unique_index ON recipe (cocktail_id, generic_ingredient_id, share, rank);
CREATE UNIQUE INDEX accessory_category_unique_name_index ON accessory_category (UPPER(name));
CREATE UNIQUE INDEX accessory_unique_name_category_index ON accessory (category_id, UPPER(name));
CREATE UNIQUE INDEX variation_unique_index ON variation (id, cocktail_id);

END;
