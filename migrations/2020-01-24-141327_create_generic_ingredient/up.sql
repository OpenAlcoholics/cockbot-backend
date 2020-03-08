CREATE TABLE generic_ingredient
(
    id           SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name         varchar(255)              NOT NULL,
    description  varchar(2048),
    image_link   varchar(512),
    is_alcoholic boolean                   NOT NULL
);
