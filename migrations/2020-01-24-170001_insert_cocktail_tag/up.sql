BEGIN TRANSACTION;

INSERT INTO cocktail_tag (tag_id, cocktail_id)
VALUES ((SELECT id FROM tag WHERE tag.name = 'Highballs'),
        (SELECT id FROM cocktail WHERE cocktail.name = 'Vodka-O'));

END;
