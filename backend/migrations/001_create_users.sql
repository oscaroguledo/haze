CREATE TABLE users (
    id UUID PRIMARY KEY,               -- Unique identifier for each user
    name VARCHAR(255) NOT NULL,         -- User's name (non-nullable)
    email VARCHAR(255) NOT NULL UNIQUE, -- User's email (non-nullable and unique)
    password VARCHAR(255) NOT NULL,     -- User's password (non-nullable)
    lastseen BOOLEAN,                   -- Last seen flag (nullable)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Timestamp when the user is created (defaults to current time)
);
