CREATE TABLE recipe_ingredient_suggestion
(
    cocktail_id            integer NOT NULL,
    ingredient_category_id integer NOT NULL,
    ingredient_id          integer NOT NULL,
    PRIMARY KEY (cocktail_id, ingredient_category_id, ingredient_id),
    FOREIGN KEY (cocktail_id, ingredient_category_id) REFERENCES recipe (cocktail_id, ingredient_category_id) ON UPDATE CASCADE

);
