-- Add migration script here
CREATE TABLE places (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nama VARCHAR(255) NOT NULL,
    deskripsi TEXT,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    ditambahkan_oleh_user_id UUID REFERENCES users(id) ON DELETE SET NULL, -- Opsional
    ditambahkan_pada TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    diperbarui_pada TIMESTAMPTZ NOT NULL DEFAULT NOW()
);