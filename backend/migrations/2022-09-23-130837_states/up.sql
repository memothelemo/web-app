CREATE TABLE states(
    id INTEGER DEFAULT 1 PRIMARY KEY NOT NULL,
    available BOOLEAN NOT NULL
);

INSERT INTO states(available) VALUES (true);
