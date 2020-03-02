CREATE TABLE tag
(
    id          varchar(255) UNIQUE PRIMARY KEY NOT NULL,
    name        varchar(255)                    NOT NULL,
    description text
);
