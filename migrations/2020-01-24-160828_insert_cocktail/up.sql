BEGIN TRANSACTION;

INSERT INTO cocktail (name, image_link, description, revision_date, notes, category_id, glass_id, ice_cubes)
VALUES ('Gin & Tonic',
        'https://images.unsplash.com/photo-1536942192778-d9aabc82a47d?ixlib=rb-1.2.1&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=1080&fit=max&ixid=eyJhcHBfaWQiOjEyMDd9',
        'Classic and easy, the gin and tonic is light and refreshing. It is a simple mixed drink—requiring just the two ingredients—and is perfect for happy hour, dinner, or anytime you simply want an invigorating beverage.',
        1535500800,
        'Hendricks Gin is best paired with a cucumber and a mold tonic water.',
        (SELECT id FROM cocktail_category WHERE cocktail_category.name = 'Highballs'),
        (SELECT id FROM glass WHERE glass.name = 'Highball'),
        true);

INSERT INTO cocktail (name, image_link, description, revision_date, notes, category_id, glass_id, ice_cubes)
VALUES ('Vodka-O',
        'http://images.unsplash.com/photo-1542800926-98fa60157353?ixlib=rb-1.2.1&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=1080&fit=max&ixid=eyJhcHBfaWQiOjEyMDd9',
        'Vodka and orange juice.',
        1535587200,
        'Not technically a cocktail.',
        (SELECT id FROM cocktail_category WHERE cocktail_category.name = 'Highballs'), -- Is this correct?
        (SELECT id FROM glass WHERE glass.name = 'Highball'),
        true);

INSERT INTO cocktail (name, image_link, description, revision_date, notes, category_id, glass_id, ice_cubes)
VALUES ('Virgin Vodka-O',
        'http://images.unsplash.com/photo-1542800926-98fa60157353?ixlib=rb-1.2.1&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=1080&fit=max&ixid=eyJhcHBfaWQiOjEyMDd9',
        'Vodka and orange juice, sans Vodka.',
        1536537600,
        'Why dont you make this one yourself?',
        (SELECT id FROM cocktail_category WHERE cocktail_category.name = 'Highballs'), -- Is this correct?
        (SELECT id FROM glass WHERE glass.name = 'Highball'),
        true);

INSERT INTO cocktail (name, image_link, description, revision_date, notes, category_id, glass_id, ice_cubes)
VALUES ('Mojito',
        'https://images.unsplash.com/photo-1531387367216-681093c0279b?ixlib=rb-0.3.5&ixid=eyJhcHBfaWQiOjEyMDd9&s=73ca72de11ae9cd2fa7367dc1ee76705&auto=format&fit=crop&w=1016&q=80',
        'Mojito is a traditional Cuban highball. Traditionally, a mojito is a cocktail that consists of five ingredients: white rum, sugar, lime juice, soda water, and mint.',
        1540833962,
        'Minty',
        (SELECT id FROM cocktail_category WHERE cocktail_category.name = 'Highballs'), -- Is this correct?
        (SELECT id FROM glass WHERE glass.name = 'Highball'),
        true);

INSERT INTO cocktail (name, image_link, description, revision_date, notes, category_id, glass_id, ice_cubes)
VALUES ('Long Island Ice Tea',
        'https://images.unsplash.com/photo-1537401198317-1231361cbd5f?ixlib=rb-0.3.5&ixid=eyJhcHBfaWQiOjEyMDd9&s=5a9460422cdba960d32b584a568f3022&auto=format&fit=crop&w=1050&q=80',
        'A Long Island Iced Tea is a type of alcoholic mixed drink typically made with vodka, tequila, light rum, triple sec, gin, and a splash of cola, which gives the drink the same amber hue as its namesake.',
        1540833962,
        'Best fucking cocktail',
        (SELECT id FROM cocktail_category WHERE cocktail_category.name = 'Highballs'), -- Is this correct?
        (SELECT id FROM glass WHERE glass.name = 'Highball'),
        true);

INSERT INTO cocktail (name, image_link, description, revision_date, notes, category_id, glass_id, ice_cubes)
VALUES ('Tequila Sunrise',
        'https://images.unsplash.com/photo-1498956405005-42073c453e8a?ixlib=rb-0.3.5&ixid=eyJhcHBfaWQiOjEyMDd9&s=1eda0d515a7d535e6d89a87e936f2628&auto=format&fit=crop&w=1051&q=80',
        'The Tequila Sunrise is a cocktail made of tequila, orange juice, and grenadine syrup and served unmixed in a tall glass.',
        1540833962,
        'Get wasted',
        (SELECT id FROM cocktail_category WHERE cocktail_category.name = 'Highballs'), -- Is this correct?
        (SELECT id FROM glass WHERE glass.name = 'Highball'),
        true);

END;
