CREATE TABLE variation
(
    id          SERIAL UNIQUE PRIMARY KEY NOT NULL,
    cocktail_id INTEGER                   NOT NULL,
    description Text,
    FOREIGN KEY (cocktail_id) REFERENCES cocktail (id)
);
