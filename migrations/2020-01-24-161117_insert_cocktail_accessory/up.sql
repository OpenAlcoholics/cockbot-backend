BEGIN TRANSACTION;

INSERT INTO cocktail_accessory (cocktail_id, accessory_category_id, pieces)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Gin & Tonic'),
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Cucumber'),
        2);

INSERT INTO cocktail_accessory (cocktail_id, accessory_category_id, pieces)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Mojito'),
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Sugar'),
        2);

INSERT INTO cocktail_accessory (cocktail_id, accessory_category_id, pieces)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Mojito'),
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Mint'),
        10);

INSERT INTO cocktail_accessory (cocktail_id, accessory_category_id, pieces)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Mojito'),
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Lemon'),
        2);

INSERT INTO cocktail_accessory (cocktail_id, accessory_category_id, pieces)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Lemon'),
        2);

INSERT INTO cocktail_accessory (cocktail_id, accessory_category_id, pieces)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Tequila Sunrise'),
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Orange'),
        1);

END;
