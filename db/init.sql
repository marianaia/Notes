CREATE DATABASE demo_db;

\c demo_db;

CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    task TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO notes (title, task)
VALUES
('Grocery Shopping', 'Buy milk, bread, eggs, and cheese'),
('Workout Plan', 'Monday: Chest and Triceps, Tuesday: Back and Biceps'),
('Meeting Notes', 'Discuss quarterly budget and project timelines')


