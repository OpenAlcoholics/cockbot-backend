BEGIN TRANSACTION;

INSERT INTO glass (name, estimated_size, image_link)
VALUES ('Highball',
        400,
        'https://cdn.shopify.com/s/files/1/0583/3185/files/liber_gin_tonic_medium.jpg?v=1500514523');

INSERT INTO glass (name, estimated_size, image_link)
VALUES ('Martini',
        250,
        'https://images.unsplash.com/photo-1444728399417-08d2aa39e6f4?ixlib=rb-0.3.5&ixid=eyJhcHBfaWQiOjEyMDd9&s=a75d09107dd70d0e844460c45024ca3c&auto=format&fit=crop&w=1950&q=80');

INSERT INTO glass (name, estimated_size, image_link)
VALUES ('Coupe',
        200,
        'https://upload.wikimedia.org/wikipedia/commons/thumb/7/70/Margarita_Glass_%28Saucer%29.svg/2000px-Margarita_Glass_%28Saucer%29.svg.png');

INSERT INTO glass (name, estimated_size, image_link)
VALUES ('Hurricane',
        400,
        'https://upload.wikimedia.org/wikipedia/commons/thumb/2/29/Hurricane_Glass.svg/2000px-Hurricane_Glass.svg.png');

INSERT INTO glass (name, estimated_size, image_link)
VALUES ('Tiki',
        400,
        'https://images.unsplash.com/photo-1526487995276-56ce643ba8a9?ixlib=rb-0.3.5&ixid=eyJhcHBfaWQiOjEyMDd9&s=9da072d38594c6e715a2889898ebf5d6&auto=format&fit=crop&w=1780&q=80');

INSERT INTO glass (name, estimated_size, image_link)
VALUES ('Tumblers',
        300,
        'https://upload.wikimedia.org/wikipedia/commons/8/84/Tumbler_MET_DP206762.jpg');

END;
