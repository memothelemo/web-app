CREATE TABLE users(
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    name VARCHAR(20) NOT NULL,
    password TEXT NOT NULL,
    moderator BOOLEAN DEFAULT false,
    viewer BOOLEAN DEFAULT false
);
