BEGIN TRANSACTION;

INSERT INTO variation_accessory (variation_id, accessory_id)
VALUES (1, (SELECT id FROM accessory WHERE name = 'Grapefruit slice'));

INSERT INTO variation_accessory (variation_id, accessory_id)
VALUES (1, (SELECT id FROM accessory WHERE name = 'Blueberry'));

END;
