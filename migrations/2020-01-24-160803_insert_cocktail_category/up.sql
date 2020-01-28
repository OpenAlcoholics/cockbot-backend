BEGIN TRANSACTION;

INSERT INTO cocktail_category (name, description, image_link)
VALUES ('Highballs',
        'Highballs are called long drinks as they are usually a sour or stirred base that is lengthened by the addition of carbonated water. The base may be sometimes be shaken before the addition of carbonated water or built in the glass and stirred to combine. Highballs are served in a tall glass over ice. Iconic highballs include the Gin & Tonic, the Moscow Mule, and the Mojito.',
        'https://images.unsplash.com/photo-1497534446932-c925b458314e?ixlib=rb-0.3.5&ixid=eyJhcHBfaWQiOjEyMDd9&s=a377d43aec8066483a03693aaf72f334&auto=format&fit=crop&w=1007&q=80');

INSERT INTO cocktail_category (name, description, image_link)
VALUES ('Sours',
        'Sours are mixed drinks containing a base liquor, lemon or lime juice, and a sweetener (triple sec, simple syrup, grenadine, or pineapple juice are common).[2] Egg whites are also included in some sours.',
        'TODO');

INSERT INTO cocktail_category (name, description, image_link)
VALUES ('Spirit-Forward',
        'TODO',
        'TODO');

INSERT INTO cocktail_category (name, description, image_link)
VALUES ('Duos and Trios',
        'A duo contains a spirit and a liqueur; a trio additionally contains a creamy ingredient, commonly cream or Irish cream.',
        'TODO');

INSERT INTO cocktail_category (name, description, image_link)
VALUES ('Champagne',
        'A champagne cocktail is an alcoholic drink made with sugar, Angostura bitters, Champagne, brandy and a maraschino cherry as a garnish.',
        'https://images.unsplash.com/photo-1526894198609-10b3cdf45c52?ixlib=rb-0.3.5&ixid=eyJhcHBfaWQiOjEyMDd9&s=2d0ea12187ea994d768fa2692230e94a&auto=format&fit=crop&w=1927&q=80');

INSERT INTO cocktail_category (name, description, image_link)
VALUES ('Tropical',
        'TODO',
        'https://images.unsplash.com/photo-1486947799489-3fabdd7d32a6?ixlib=rb-0.3.5&ixid=eyJhcHBfaWQiOjEyMDd9&s=9cbb531631c97478f94c71d68200b654&auto=format&fit=crop&w=975&q=80');

INSERT INTO cocktail_category (name, description, image_link)
VALUES ('Beer',
        'Beer is brewed from cereal grains—most commonly from malted barley, though wheat, maize (corn), and rice are also used.',
        'https://images.unsplash.com/photo-1532377611310-4564e426e7c1?ixlib=rb-0.3.5&ixid=eyJhcHBfaWQiOjEyMDd9&s=363f78001548aab97d28ccabe24b6ca0&auto=format&fit=crop&w=934&q=80');

INSERT INTO cocktail_category (name, description, image_link)
VALUES ('Wine',
        'Yeast consumes the sugar in the grapes and converts it to ethanol, carbon dioxide, and heat. Different varieties of grapes and strains of yeasts produce different styles of wine.',
        'https://images.unsplash.com/photo-1474722883778-792e7990302f?ixlib=rb-0.3.5&ixid=eyJhcHBfaWQiOjEyMDd9&s=393916ed293a0f729929fbcb1d8f34c2&auto=format&fit=crop&w=937&q=80');

INSERT INTO cocktail_category (name, description, image_link)
VALUES ('Hot',
        'TODO',
        'TODO');

INSERT INTO cocktail_category (name, description, image_link)
VALUES ('Creamy',
        'TODO',
        'TODO');

END;
