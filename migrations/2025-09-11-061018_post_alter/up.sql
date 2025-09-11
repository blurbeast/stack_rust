-- Your SQL goes here


ALTER TABLE posts ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW(),
ADD COLUMN user_id INT NOT NULL REFERENCES users(id) default 1;