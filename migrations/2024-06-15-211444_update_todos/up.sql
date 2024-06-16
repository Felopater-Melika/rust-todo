-- Your SQL goes here
ALTER TABLE todos
    ADD COLUMN description TEXT;
ALTER TABLE todos
    ADD COLUMN due_date TIMESTAMP;