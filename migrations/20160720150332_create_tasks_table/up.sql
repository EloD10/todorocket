CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 'false'
);

INSERT INTO public.tasks(description) VALUES 
  ('task 1'),
  ('task 2')