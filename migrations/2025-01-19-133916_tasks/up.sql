-- Your SQL goes here
CREATE TABLE Task (
    id TEXT NOT NULL PRIMARY KEY,
    description TEXT NOT NULL,
    completed BOOLEAN DEFAULT FALSE
);
