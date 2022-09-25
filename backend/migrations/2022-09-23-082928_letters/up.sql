CREATE TABLE letters(
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
  created_at TIMESTAMP DEFAULT now() NOT NULL,
  author VARCHAR(50) UNIQUE NOT NULL,
  message TEXT NOT NULL,
  secret BOOLEAN DEFAULT false NOT NULL
);
