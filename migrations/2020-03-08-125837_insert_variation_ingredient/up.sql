BEGIN TRANSACTION;

INSERT INTO variation_ingredient (variation_id, ingredient_id)
VALUES (1, (SELECT id FROM ingredient WHERE name = 'Brockmans Gin'));

END;
