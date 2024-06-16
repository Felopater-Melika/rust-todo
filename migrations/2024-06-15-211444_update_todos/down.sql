-- This file should undo anything in `up.sql`
ALTER TABLE todos
    DROP COLUMN description;

ALTER TABLE todos
    DROP COLUMN due_date;

