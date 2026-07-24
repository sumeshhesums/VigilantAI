CREATE TYPE notification_channel AS ENUM ('email', 'webhook');
CREATE TYPE notification_status AS ENUM ('pending', 'sent', 'failed', 'retrying');

CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    incident_id UUID NOT NULL REFERENCES incidents(id) ON DELETE CASCADE,
    channel notification_channel NOT NULL,
    recipient VARCHAR(500) NOT NULL,
    status notification_status NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    response_code INTEGER,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    sent_at TIMESTAMPTZ
);

CREATE INDEX idx_notifications_incident_id ON notifications (incident_id);
CREATE INDEX idx_notifications_status ON notifications (status);
CREATE INDEX idx_notifications_channel ON notifications (channel);
CREATE INDEX idx_notifications_created_at ON notifications (created_at DESC);
