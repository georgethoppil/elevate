-- Add migration script here
CREATE TABLE games (
   game_id uuid PRIMARY KEY,
   user_id uuid REFERENCES users(user_id) ON DELETE CASCADE,
   type TEXT NOT NULL,
   occurred_at TIMESTAMP NOT NULL
);
