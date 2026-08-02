-- Seed the system roles and bootstrap users.
--
-- Bootstrap users (passwords are development-only and must be rotated):
--   admin@vigilantai.local   / Admin@VigilantAI2026     (system_admin)
--   gateway@vigilantai.local / Gateway@VigilantAI2026    (api_integration)
--
-- The gateway@vigilantai.local account is the dedicated service account used by
-- the camera gateway to create incidents, upload evidence, and send
-- notifications. All statements are idempotent so the migration can be re-run.

INSERT INTO roles (name, description) VALUES
    ('system_admin', 'Full system administration'),
    ('security_admin', 'Security administration'),
    ('security_analyst', 'Security analysis'),
    ('operator', 'Operations'),
    ('viewer', 'Read-only viewer'),
    ('api_integration', 'API integration service account')
ON CONFLICT (name) DO NOTHING;

INSERT INTO users (email, password_hash, first_name, last_name) VALUES
    (
        'admin@vigilantai.local',
        '$argon2id$v=19$m=19456,t=2,p=1$9M3MaVX3LPllozOUuyAM0g$Q/w64Ni7p55iDg6cS4nVJoVsT3xegdXXW18QbsEQ3VY',
        'System',
        'Administrator'
    ),
    (
        'gateway@vigilantai.local',
        '$argon2id$v=19$m=19456,t=2,p=1$1uTOFBB6j0bNMDYMtQ0mCQ$sYDdG2L/GVGpCqhZZL4zNuMR85CK/VyN0KgZ52RAKfk',
        'Camera',
        'Gateway'
    )
ON CONFLICT (email) DO NOTHING;

INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id FROM users u, roles r
WHERE u.email = 'admin@vigilantai.local' AND r.name = 'system_admin'
ON CONFLICT DO NOTHING;

INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id FROM users u, roles r
WHERE u.email = 'gateway@vigilantai.local' AND r.name = 'api_integration'
ON CONFLICT DO NOTHING;
