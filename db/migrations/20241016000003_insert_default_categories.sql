-- migrate:up
-- Add some default accounting categories
INSERT INTO accounting_categories (name) VALUES 
    ('Development'),
    ('Meetings'),
    ('Documentation'),
    ('Testing'),
    ('Code Review'),
    ('Planning'),
    ('Research'),
    ('Support'),
    ('Administration');

-- migrate:down
-- Remove the default categories (optional - you might want to keep them)
DELETE FROM accounting_categories WHERE name IN (
    'Development',
    'Meetings', 
    'Documentation',
    'Testing',
    'Code Review',
    'Planning',
    'Research',
    'Support',
    'Administration'
);