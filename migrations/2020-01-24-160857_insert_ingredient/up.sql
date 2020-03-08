BEGIN TRANSACTION;

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Hendricks Gin',
        'https://upload.wikimedia.org/wikipedia/commons/2/25/Gin_Hendricks.jpg',
        'Earthy',
        44,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Gin'));

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Tonic Water',
        'https://cdn.pixabay.com/photo/2017/05/10/08/05/gin-2300126_960_720.png',
        '',
        0,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Tonic Water'));

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Cucumber slice',
        'https://c1.staticflickr.com/4/3234/2738586453_f23bc8244e_b.jpg',
        'Slice of a cucumber',
        0,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Tonic Water'));

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Orange Juice',
        'TODO',
        'TODO',
        0,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Orange Juice'));

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Three Sixty Vodka',
        'TODO',
        'TODO',
        37,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Vodka'));

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Tequila',
        'TODO',
        'TODO',
        37,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Vodka'));

END;
