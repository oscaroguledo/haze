
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    sender_id UUID REFERENCES users(id),
    content TEXT NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);