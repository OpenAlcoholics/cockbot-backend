BEGIN TRANSACTION;

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Brockmans Gin',
        'https://cdn1.masterofmalt.com/gin/p-2813/brockmans/brockmans-intensely-smooth-gin.jpg?ss=2.0',
        'Fruity',
        40,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Gin'));

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Hendricks Gin',
        'https://upload.wikimedia.org/wikipedia/commons/2/25/Gin_Hendricks.jpg',
        'Earthy',
        44,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Gin'));

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Orange Juice',
        'TODO',
        'TODO',
        0,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Orange Juice'));

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Tonic Water',
        'https://cdn.pixabay.com/photo/2017/05/10/08/05/gin-2300126_960_720.png',
        '',
        0,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Tonic Water'));

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Three Sixty Vodka',
        'TODO',
        'TODO',
        37,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Vodka'));

INSERT INTO ingredient (name, image_link, notes, alcohol_percentage, generic_ingredient_id)
VALUES ('Tequila Silver',
        'TODO',
        'TODO',
        37,
        (SELECT id FROM generic_ingredient WHERE generic_ingredient.name = 'Tequila'));

END;
