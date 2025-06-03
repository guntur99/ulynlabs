-- Add migration script here
ALTER TABLE places
ADD COLUMN deleted_at TIMESTAMP NULL;