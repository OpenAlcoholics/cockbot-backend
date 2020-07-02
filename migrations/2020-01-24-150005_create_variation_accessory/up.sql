CREATE TABLE variation_accessory
(
    variation_id SERIAL  NOT NULL,
    accessory_id INTEGER NOT NULL,
    PRIMARY KEY (variation_id, accessory_id),
    FOREIGN KEY (accessory_id) REFERENCES ingredient (id),
    FOREIGN KEY (variation_id) REFERENCES variation (id)
);
