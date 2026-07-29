-- ============================================================
-- VigilantAI Seed Data
-- ============================================================
-- This script populates the database with initial data:
--   - Default roles (system_admin, security_admin, security_analyst, operator, viewer, api_integration)
--   - Default permissions (23 resource:action pairs)
--   - Default role-permission mappings
--   - Admin user (email: admin@vigilantai.local, password: admin123)
-- ============================================================

BEGIN;

-- ── Roles ────────────────────────────────────────────────────
INSERT INTO roles (id, name, description) VALUES
    (gen_random_uuid(), 'system_admin',      'Full system access with all permissions'),
    (gen_random_uuid(), 'security_admin',    'Security administration, user management, and configuration'),
    (gen_random_uuid(), 'security_analyst',  'Incident analysis, evidence review, and reporting'),
    (gen_random_uuid(), 'operator',          'Day-to-day monitoring and incident response'),
    (gen_random_uuid(), 'viewer',            'Read-only access to cameras, incidents, and dashboards'),
    (gen_random_uuid(), 'api_integration',   'Programmatic access for external system integration')
ON CONFLICT (name) DO NOTHING;

-- ── Permissions ──────────────────────────────────────────────
INSERT INTO permissions (id, name, description) VALUES
    (gen_random_uuid(), 'user:view',        'View user profiles and details'),
    (gen_random_uuid(), 'user:create',      'Create new user accounts'),
    (gen_random_uuid(), 'user:update',      'Modify user account details'),
    (gen_random_uuid(), 'user:delete',      'Remove user accounts'),
    (gen_random_uuid(), 'role:view',        'View role definitions and assignments'),
    (gen_random_uuid(), 'role:update',      'Modify role definitions and assignments'),
    (gen_random_uuid(), 'camera:view',      'View camera list and stream status'),
    (gen_random_uuid(), 'camera:create',    'Register new cameras'),
    (gen_random_uuid(), 'camera:update',    'Modify camera configuration'),
    (gen_random_uuid(), 'camera:delete',    'Remove cameras from the system'),
    (gen_random_uuid(), 'incident:view',    'View incident details and history'),
    (gen_random_uuid(), 'incident:create',  'Create new incident records'),
    (gen_random_uuid(), 'incident:update',  'Modify incident details'),
    (gen_random_uuid(), 'incident:close',   'Close or resolve incidents'),
    (gen_random_uuid(), 'evidence:view',    'View evidence files and metadata'),
    (gen_random_uuid(), 'evidence:download','Download evidence files'),
    (gen_random_uuid(), 'evidence:upload',  'Upload evidence files'),
    (gen_random_uuid(), 'evidence:delete',  'Remove evidence files'),
    (gen_random_uuid(), 'notification:view','View notification history'),
    (gen_random_uuid(), 'notification:send','Send notifications'),
    (gen_random_uuid(), 'notification:retry','Retry failed notifications'),
    (gen_random_uuid(), 'dashboard:view',   'View system dashboards'),
    (gen_random_uuid(), 'system:admin',     'Full system administration access')
ON CONFLICT (name) DO NOTHING;

-- ── Role-Permission Mappings ─────────────────────────────────
-- SystemAdmin: ALL permissions
WITH role_cte AS (SELECT id FROM roles WHERE name = 'system_admin')
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM role_cte r, permissions p
ON CONFLICT DO NOTHING;

-- SecurityAdmin: user:*, role:*, camera:*, incident:*, evidence:*, notification:*, dashboard:view
WITH role_cte AS (SELECT id FROM roles WHERE name = 'security_admin')
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM role_cte r, permissions p
WHERE p.name IN (
    'user:view', 'user:create', 'user:update', 'user:delete',
    'role:view', 'role:update',
    'camera:view', 'camera:create', 'camera:update', 'camera:delete',
    'incident:view', 'incident:create', 'incident:update', 'incident:close',
    'evidence:view', 'evidence:download', 'evidence:upload', 'evidence:delete',
    'notification:view', 'notification:send', 'notification:retry',
    'dashboard:view'
)
ON CONFLICT DO NOTHING;

-- SecurityAnalyst: incident:*, evidence:view/download, notification:view, camera:view, dashboard:view
WITH role_cte AS (SELECT id FROM roles WHERE name = 'security_analyst')
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM role_cte r, permissions p
WHERE p.name IN (
    'camera:view',
    'incident:view', 'incident:create', 'incident:update', 'incident:close',
    'evidence:view', 'evidence:download',
    'notification:view', 'notification:retry',
    'dashboard:view'
)
ON CONFLICT DO NOTHING;

-- Operator: camera:view/update, incident:view/update, evidence:view/upload, notification:view/retry, dashboard:view
WITH role_cte AS (SELECT id FROM roles WHERE name = 'operator')
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM role_cte r, permissions p
WHERE p.name IN (
    'camera:view', 'camera:update',
    'incident:view', 'incident:update',
    'evidence:view', 'evidence:upload',
    'notification:view', 'notification:retry',
    'dashboard:view'
)
ON CONFLICT DO NOTHING;

-- Viewer: camera:view, incident:view, evidence:view, notification:view, dashboard:view
WITH role_cte AS (SELECT id FROM roles WHERE name = 'viewer')
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM role_cte r, permissions p
WHERE p.name IN (
    'camera:view',
    'incident:view',
    'evidence:view',
    'notification:view',
    'dashboard:view'
)
ON CONFLICT DO NOTHING;

-- ApiIntegration: camera:view/create, incident:view/create, evidence:view/upload, notification:view/send
WITH role_cte AS (SELECT id FROM roles WHERE name = 'api_integration')
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM role_cte r, permissions p
WHERE p.name IN (
    'camera:view', 'camera:create',
    'incident:view', 'incident:create',
    'evidence:view', 'evidence:upload',
    'notification:view', 'notification:send'
)
ON CONFLICT DO NOTHING;

-- ── Admin User ───────────────────────────────────────────────
-- Password: admin123 (Argon2id hash)
-- This is a DEVELOPMENT-ONLY credential. Change immediately in production.
INSERT INTO users (id, email, password_hash, first_name, last_name, is_active)
VALUES (
    gen_random_uuid(),
    'admin@vigilantai.local',
    '$argon2id$v=19$m=19456,t=2,p=1$RANDOMSALT$HASHPLACEHOLDER',
    'System',
    'Admin',
    TRUE
)
ON CONFLICT (email) DO NOTHING;

-- Assign system_admin role to admin user
WITH
    admin_user AS (SELECT id FROM users WHERE email = 'admin@vigilantai.local'),
    admin_role AS (SELECT id FROM roles WHERE name = 'system_admin')
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id
FROM admin_user u, admin_role r
ON CONFLICT DO NOTHING;

COMMIT;
