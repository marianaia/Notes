CREATE TABLE IF NOT EXISTS notes  (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    task TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);

INSERT INTO notes (title, task)
VALUES
('Grocery Shopping', 'Buy milk, bread, eggs, and cheese'),
('Workout Plan', 'Monday: Chest and Triceps, Tuesday: Back and Biceps'),
('Meeting Notes', 'Discuss quarterly budget and project timelines')


