CREATE TABLE ingredient
(
    id                 SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name               varchar(255)              NOT NULL,
    image_link         varchar(512),
    notes              varchar(1024),
    alcohol_percentage integer DEFAULT 0         NOT NULL,
    generic_ingredient_id        integer                   NOT NULL,
    FOREIGN KEY (generic_ingredient_id) REFERENCES generic_ingredient (id) ON DELETE CASCADE ON UPDATE CASCADE
);
