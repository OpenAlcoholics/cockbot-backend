CREATE TABLE recipe
(
    cocktail_id            integer NOT NULL,
    generic_ingredient_id integer NOT NULL,
    share                  integer NOT NUll,
    rank                   integer, -- The order in which the ingredients should be put into the glass
    PRIMARY KEY (cocktail_id, generic_ingredient_id),
    CHECK (share >= 0 AND share <= 100),
    FOREIGN KEY (cocktail_id) REFERENCES cocktail (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (generic_ingredient_id) REFERENCES generic_ingredient (id) ON DELETE CASCADE ON UPDATE CASCADE
);
