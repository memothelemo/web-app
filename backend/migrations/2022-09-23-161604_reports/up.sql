CREATE TABLE reports(
    id BIGSERIAL PRIMARY KEY NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    letter_id uuid NOT NULL,
    type INTEGER NOT NULL,
    details TEXT NOT NULL,
    resolved BOOLEAN NOT NULL DEFAULT false,

    FOREIGN KEY(letter_id) REFERENCES letters(id)
);
