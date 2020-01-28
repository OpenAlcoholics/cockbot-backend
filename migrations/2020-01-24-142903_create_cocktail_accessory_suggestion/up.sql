CREATE TABLE cocktail_accessory_suggestion
(
    cocktail_id           integer NOT NULL,
    accessory_category_id integer NOT NULL,
    accessory_id          integer NOT NULL,
    PRIMARY KEY (cocktail_id, accessory_category_id, accessory_id),
    FOREIGN KEY (cocktail_id, accessory_category_id) REFERENCES cocktail_accessory (cocktail_id, accessory_category_id) ON UPDATE CASCADE
);
