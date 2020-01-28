CREATE TABLE ingredient
(
    id                 SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name               varchar(255)              NOT NULL,
    image_link         varchar(512),
    notes              varchar(1024),
    alcohol_percentage integer DEFAULT 0         NOT NULL,
    category_id        integer                   NOT NULL,
    FOREIGN KEY (category_id) REFERENCES ingredient_category (id) ON DELETE CASCADE ON UPDATE CASCADE
);
