-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE projects (
    id uuid default uuid_generate_v4() PRIMARY KEY,
    author_id uuid,
    forked_from_id uuid,
    title VARCHAR(50) NOT NULL,
    description TEXT,
    files JSON NOT NULL,
    layout JSON,
    config JSON,
    published BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP(3) NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT current_timestamp,
    CONSTRAINT author_id FOREIGN KEY(author_id) REFERENCES users(id) ON DELETE SET NULL
);