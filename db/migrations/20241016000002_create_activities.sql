-- migrate:up
CREATE TABLE activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date DATE NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME,
    category_id UUID NOT NULL REFERENCES accounting_categories(id) ON DELETE CASCADE,
    task TEXT NOT NULL,
    comment TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better query performance
CREATE INDEX idx_activities_date ON activities(date);
CREATE INDEX idx_activities_category_id ON activities(category_id);
CREATE INDEX idx_activities_date_category ON activities(date, category_id);

-- migrate:down
DROP TABLE IF EXISTS activities;