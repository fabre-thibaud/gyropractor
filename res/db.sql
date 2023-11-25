CREATE TABLE IF NOT EXISTS alerts
(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255),
    component VARCHAR(255),
    checked boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    checked_at timestamp with time zone DEFAULT NULL
);
