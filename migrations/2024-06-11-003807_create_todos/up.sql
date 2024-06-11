-- Your SQL goes here
CREATE TABLE todos
(
    id         SERIAL PRIMARY KEY,
    title      VARCHAR   NOT NULL,
    completed  BOOLEAN   NOT NULL DEFAULT FALSE,
    user_id    INTEGER   NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id)
);