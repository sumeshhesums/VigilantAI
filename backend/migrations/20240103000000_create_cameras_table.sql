CREATE TABLE cameras (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(150) NOT NULL,
    location VARCHAR(255),
    rtsp_url TEXT NOT NULL UNIQUE,
    status VARCHAR(30) NOT NULL DEFAULT 'offline',
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    fps INTEGER,
    resolution VARCHAR(30),
    last_seen TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cameras_name ON cameras (name);
CREATE INDEX idx_cameras_status ON cameras (status);
