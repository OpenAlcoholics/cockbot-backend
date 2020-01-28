CREATE TABLE cocktail_category
(
    id          SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name        varchar(255)              NOT NULL,
    description varchar(512),
    image_link  varchar(512)
);
