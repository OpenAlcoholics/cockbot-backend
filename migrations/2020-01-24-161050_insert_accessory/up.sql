BEGIN TRANSACTION;

INSERT INTO accessory (name, description, image_link, category_id)
VALUES ('Cucumber slice',
        'Slice of a cucumber',
        'https://c1.staticflickr.com/4/3234/2738586453_f23bc8244e_b.jpg',
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Cucumber'));

INSERT INTO accessory (name, description, image_link, category_id)
VALUES ('Lemon Slice',
        'Slice of a lemon',
        '',
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Lemon'));

INSERT INTO accessory (name, description, image_link, category_id)
VALUES ('Orange Slice',
        'Slice of an orange',
        '',
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Citrus'));

INSERT INTO accessory (name, description, image_link, category_id)
VALUES ('White sugar',
        '...',
        '',
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Sugar'));

INSERT INTO accessory (name, description, image_link, category_id)
VALUES ('Brown sugar',
        '...',
        '',
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Sugar'));

INSERT INTO accessory (name, description, image_link, category_id)
VALUES ('Mint',
        '...',
        '',
        (SELECT id FROM accessory_category WHERE accessory_category.name = 'Herbs'));

END;
