BEGIN TRANSACTION;
INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Tonic Water',
        'Tonic water (or Indian tonic water) is a carbonated soft drink in which quinine is dissolved. Originally used as a prophylactic against malaria, tonic water usually now has a significantly lower quinine content and is consumed for its distinctive bitter flavor. It is often used in mixed drinks, particularly in gin and tonic.',
        'https://upload.wikimedia.org/wikipedia/commons/b/b2/Tonic_water_uv.jpg',
        false);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Gin',
        'Gin is liquor which derives its predominant flavour from juniper berries (Juniperus communis). Gin is one of the broadest categories of spirits, all of various origins, styles, and flavour profiles that revolve around juniper as a common ingredient.',
        'https://cdn.pixabay.com/photo/2017/05/10/08/05/gin-2300126_960_720.png',
        true);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Vodka',
        'TODO',
        'TODO',
        true);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Dark Rum',
        'TODO',
        'TODO',
        true);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('White Rum',
        'TODO',
        'TODO',
        true);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Orange Juice',
        'TODO',
        'TODO',
        false);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Soda',
        'TODO',
        'TODO',
        false);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Tequila',
        'TODO',
        'TODO',
        true);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Orange liquor',
        'TODO',
        'TODO',
        false);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Sugar syrup',
        'TODO',
        'TODO',
        false);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Lemon juice',
        'TODO',
        'TODO',
        false);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Lime juice',
        'TODO',
        'TODO',
        false);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Cola',
        'TODO',
        'TODO',
        false);

INSERT INTO ingredient_category (name, description, image_link, is_alcoholic)
VALUES ('Grenadine',
        'TODO',
        'TODO',
        false);
END;
