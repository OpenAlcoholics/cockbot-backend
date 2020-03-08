BEGIN TRANSACTION;

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Gin & Tonic'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Gin'),
        50,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Gin & Tonic'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Tonic Water'),
        50,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Vodka-O'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Orange Juice'),
        70,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Vodka-O'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Vodka'),
        30,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Virgin Vodka-O'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Orange Juice'),
        100,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Mojito'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'White Rum'), -- TODO
        30,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Mojito'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Soda'), -- TODO
        70,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Dark Rum'), -- TODO
        12,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Vodka'), -- TODO
        12,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Gin'), -- TODO
        12,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Tequila'), -- TODO
        12,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Orange liquor'), -- TODO
        11,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Sugar syrup'), -- TODO
        11,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Lemon juice'), -- TODO
        11,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Lime juice'), -- TODO
        11,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Long Island Ice Tea'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Cola'), -- TODO
        8,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Tequila Sunrise'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Tequila'),
        20,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Tequila Sunrise'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Orange Juice'),
        75,
        0);

INSERT INTO recipe (cocktail_id, generic_ingredient_id, share, rank)
VALUES ((SELECT id FROM cocktail WHERE cocktail.name = 'Tequila Sunrise'),
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Grenadine'), -- TODO
        5,
        0);

END;
