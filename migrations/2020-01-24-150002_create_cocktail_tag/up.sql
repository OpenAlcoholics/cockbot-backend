CREATE TABLE cocktail_tag
(
    tag_id      varchar(255) NOT NULL,
    cocktail_id integer      NOT NULL,
    PRIMARY KEY (tag_id, cocktail_id),
    FOREIGN KEY (tag_id) REFERENCES tag (id),
    FOREIGN KEY (cocktail_id) REFERENCES cocktail (id)
);
