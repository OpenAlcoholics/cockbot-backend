CREATE TABLE recipe_ingredient_suggestion
(
    cocktail_id            integer NOT NULL,
    generic_ingredient_id integer NOT NULL,
    ingredient_id          integer NOT NULL,
    PRIMARY KEY (cocktail_id, generic_ingredient_id, ingredient_id),
    FOREIGN KEY (cocktail_id, generic_ingredient_id) REFERENCES recipe (cocktail_id, generic_ingredient_id) ON UPDATE CASCADE

);
