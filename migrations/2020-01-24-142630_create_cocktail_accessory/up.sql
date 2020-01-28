CREATE TABLE cocktail_accessory
(
    cocktail_id           integer NOT NULL,
    accessory_category_id integer NOT NULL,
    pieces                integer,
    PRIMARY KEY (cocktail_id, accessory_category_id),
    FOREIGN KEY (cocktail_id) REFERENCES cocktail (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (accessory_category_id) REFERENCES accessory_category (id) ON DELETE CASCADE ON UPDATE CASCADE
);
