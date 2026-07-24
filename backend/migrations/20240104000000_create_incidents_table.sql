CREATE TYPE incident_severity AS ENUM ('low', 'medium', 'high', 'critical');
CREATE TYPE incident_status AS ENUM ('open', 'acknowledged', 'resolved', 'false_positive');

CREATE TABLE incidents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    camera_id UUID NOT NULL REFERENCES cameras(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    severity incident_severity NOT NULL DEFAULT 'medium',
    status incident_status NOT NULL DEFAULT 'open',
    event_type VARCHAR(100) NOT NULL,
    confidence NUMERIC(5,4) NOT NULL CHECK (confidence >= 0 AND confidence <= 1),
    bounding_box JSONB,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_incidents_camera_id ON incidents (camera_id);
CREATE INDEX idx_incidents_timestamp ON incidents (timestamp DESC);
CREATE INDEX idx_incidents_severity ON incidents (severity);
CREATE INDEX idx_incidents_status ON incidents (status);
CREATE INDEX idx_incidents_event_type ON incidents (event_type);
