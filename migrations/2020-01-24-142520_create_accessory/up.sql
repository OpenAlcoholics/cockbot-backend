CREATE TABLE accessory
(
    id          SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name        varchar(255)              NOT NULL,
    description varchar(512),
    image_link  varchar(512),
    category_id integer                   NOT NULL,
    FOREIGN KEY (category_id) REFERENCES accessory_category (id) ON DELETE CASCADE ON UPDATE CASCADE
);
