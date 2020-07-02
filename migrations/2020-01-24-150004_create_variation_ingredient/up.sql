CREATE TABLE variation_ingredient
(
    variation_id  SERIAL  NOT NULL,
    ingredient_id INTEGER NOT NULL,
    PRIMARY KEY (variation_id, ingredient_id),
    FOREIGN KEY (ingredient_id) REFERENCES ingredient (id),
    FOREIGN KEY (variation_id) REFERENCES variation (id)
);
