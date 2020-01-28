CREATE TABLE cocktail
(
    id            SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name          varchar(255)              NOT NULL,
    image_link    varchar(512),
    description   varchar(1024),
    revision_date integer                   NOT NULL, -- unix timestamp
    notes         text,
    category_id   integer                   NOT NULL,
    glass_id      integer                   NOT NULL,
    ice_cubes     boolean                   NOT NULL,
    FOREIGN KEY (category_id) REFERENCES cocktail_category (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (glass_id) REFERENCES glass (id) ON DELETE CASCADE ON UPDATE CASCADE
);
