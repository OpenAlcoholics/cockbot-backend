BEGIN TRANSACTION;

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Gin & Tonic'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Gin'),
        50,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Gin & Tonic'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Tonic Water'),
        50,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Vodka-O'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Orange Juice'),
        70,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Vodka-O'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Vodka'),
        30,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Virgin Vodka-O'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Orange Juice'),
        100,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Mojito'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'White Rum'), -- TODO
        30,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Mojito'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Soda'), -- TODO
        70,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Dark Rum'), -- TODO
        12,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Vodka'), -- TODO
        12,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Gin'), -- TODO
        12,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Tequila'), -- TODO
        12,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Orange liquor'), -- TODO
        11,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Sugar syrup'), -- TODO
        11,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Lemon juice'), -- TODO
        11,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Lime juice'), -- TODO
        11,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Cola'), -- TODO
        8,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Tequila Sunrise'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Tequila'),
        20,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Tequila Sunrise'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Orange Juice'),
        75,
        0);

INSERT INTO recipe (cocktail_id, ingredient_category_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Tequila Sunrise'),
        (SELECT id FROM ingredient_category WHERE ingredient_category.name = 'Grenadine'), -- TODO
        5,
        0);

END;
