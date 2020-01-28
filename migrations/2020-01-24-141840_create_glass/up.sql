CREATE TABLE glass
(
    id             SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name           varchar(255)              NOT NULL,
    estimated_size integer, -- What unit do we use here?
    image_link     varchar(512)
);
