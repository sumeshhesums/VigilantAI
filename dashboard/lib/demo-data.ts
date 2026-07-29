import type { DashboardKPIs, LiveStats, AlertTrendSeries, IncidentSummary } from "@/hooks/use-dashboard";
import type { Role } from "@/services/role.service";
import type { Notification } from "@/types";
import type { HealthStatus } from "@/types/health";
import type { Camera } from "@/types/camera";
import type { Incident } from "@/types/incident";
import type { User } from "@/types/user";

export const DEMO_KPIs: DashboardKPIs = {
  active_cameras: 12,
  online_cameras: 10,
  offline_cameras: 2,
  total_detections_24h: 1847,
  critical_alerts: 3,
  open_incidents: 8,
  avg_response_time_seconds: 42,
  sla_compliance_percent: 94.7,
  detection_trend: "+12.3%",
};

export const DEMO_LIVE_STATS: LiveStats = {
  detections_per_minute: 24.5,
  active_alerts: 3,
  cameras_streaming: 10,
  event_queue_depth: 0,
  system_health: "healthy",
  updated_at: new Date().toISOString(),
};

function generateAlertTrends(): AlertTrendSeries[] {
  const now = new Date();
  return Array.from({ length: 24 }, (_, i) => {
    const ts = new Date(now.getTime() - (23 - i) * 3600000);
    return {
      timestamp: ts.toISOString(),
      critical: Math.floor(Math.random() * 5),
      high: Math.floor(Math.random() * 12),
      medium: Math.floor(Math.random() * 20),
      low: Math.floor(Math.random() * 30),
    };
  });
}

export const DEMO_ALERT_TRENDS = { interval: "1h", series: generateAlertTrends() };

export const DEMO_INCIDENT_SUMMARY: IncidentSummary = {
  open: 8,
  acknowledged: 3,
  investigating: 2,
  resolved: 45,
  closed: 120,
  avg_resolution_time_minutes: 28,
  overdue_count: 1,
};

export const DEMO_ROLES: Role[] = [
  { id: "role-1", name: "system_admin", description: "Full system access with all permissions" },
  { id: "role-2", name: "security_admin", description: "Security configuration and user management" },
  { id: "role-3", name: "security_analyst", description: "Incident investigation and response" },
  { id: "role-4", name: "operator", description: "Camera monitoring and basic operations" },
  { id: "role-5", name: "viewer", description: "Read-only access to dashboards and reports" },
];

export const DEMO_NOTIFICATIONS: Notification[] = [
  {
    id: "notif-1", incident_id: "inc-1", channel: "email", recipient: "admin@vigilantai.com",
    status: "sent", attempts: 1, response_code: 200, error_message: null, created_at: new Date(Date.now() - 300000).toISOString(),
    sent_at: new Date(Date.now() - 299000).toISOString(),
  },
  {
    id: "notif-2", incident_id: "inc-2", channel: "webhook", recipient: "https://hooks.slack.com/alerts",
    status: "sent", attempts: 1, response_code: 200, error_message: null, created_at: new Date(Date.now() - 600000).toISOString(),
    sent_at: new Date(Date.now() - 599000).toISOString(),
  },
  {
    id: "notif-3", incident_id: "inc-3", channel: "webhook", recipient: "analyst@vigilantai.com",
    status: "sent", attempts: 1, response_code: null, error_message: null, created_at: new Date(Date.now() - 900000).toISOString(),
    sent_at: new Date(Date.now() - 899000).toISOString(),
  },
  {
    id: "notif-4", incident_id: "inc-4", channel: "email", recipient: "ops@vigilantai.com",
    status: "failed", attempts: 3, response_code: 503, error_message: "SMTP connection refused", created_at: new Date(Date.now() - 1200000).toISOString(),
    sent_at: null,
  },
  {
    id: "notif-5", incident_id: "inc-5", channel: "email", recipient: "security@vigilantai.com",
    status: "sent", attempts: 1, response_code: 200, error_message: null, created_at: new Date(Date.now() - 1500000).toISOString(),
    sent_at: new Date(Date.now() - 1499000).toISOString(),
  },
];

export const DEMO_HEALTH: HealthStatus = {
  status: "healthy",
  version: "1.0.0",
  uptime_seconds: 259200,
  services: {
    database: { status: "healthy", latency_ms: 2 },
    cache: { status: "healthy", latency_ms: 1 },
    ai_engine: { status: "healthy", latency_ms: 45, active_streams: 0 },
    camera_gateway: { status: "healthy", latency_ms: 12, active_streams: 8 },
    evidence_store: { status: "healthy", latency_ms: 8, used_gb: 24.5 },
  },
};

export const DEMO_CAMERAS: Camera[] = [
  { id: "cam-1", name: "Lobby Main Entrance", location: "Building A - Lobby", rtsp_url: "rtsp://192.168.1.101:554/stream", status: "online", enabled: true, fps: 15, resolution: "1920x1080", last_seen: new Date().toISOString(), created_at: "2024-01-15T08:00:00Z", updated_at: "2024-01-15T08:00:00Z" },
  { id: "cam-2", name: "Parking Garage Level 1", location: "Parking Garage - L1", rtsp_url: "rtsp://192.168.1.102:554/stream", status: "online", enabled: true, fps: 10, resolution: "1280x720", last_seen: new Date().toISOString(), created_at: "2024-01-15T08:00:00Z", updated_at: "2024-01-15T08:00:00Z" },
  { id: "cam-3", name: "Server Room Entry", location: "Building B - Basement", rtsp_url: "rtsp://192.168.1.103:554/stream", status: "online", enabled: true, fps: 20, resolution: "1920x1080", last_seen: new Date().toISOString(), created_at: "2024-02-01T10:00:00Z", updated_at: "2024-02-01T10:00:00Z" },
  { id: "cam-4", name: "Warehouse Loading Dock", location: "Warehouse - Dock 3", rtsp_url: "rtsp://192.168.1.104:554/stream", status: "offline", enabled: false, fps: 15, resolution: "1920x1080", last_seen: "2024-12-01T14:30:00Z", created_at: "2024-03-10T12:00:00Z", updated_at: "2024-12-01T14:30:00Z" },
  { id: "cam-5", name: "Rooftop Perimeter East", location: "Building A - Roof", rtsp_url: "rtsp://192.168.1.105:554/stream", status: "online", enabled: true, fps: 10, resolution: "1280x720", last_seen: new Date().toISOString(), created_at: "2024-04-05T09:00:00Z", updated_at: "2024-04-05T09:00:00Z" },
  { id: "cam-6", name: "Reception Desk", location: "Building A - Floor 1", rtsp_url: "rtsp://192.168.1.106:554/stream", status: "online", enabled: true, fps: 15, resolution: "1920x1080", last_seen: new Date().toISOString(), created_at: "2024-04-20T11:00:00Z", updated_at: "2024-04-20T11:00:00Z" },
  { id: "cam-7", name: "Emergency Exit North", location: "Building A - Floor 2", rtsp_url: "rtsp://192.168.1.107:554/stream", status: "maintenance", enabled: true, fps: 15, resolution: "1920x1080", last_seen: "2024-11-28T16:00:00Z", created_at: "2024-05-01T14:00:00Z", updated_at: "2024-11-28T16:00:00Z" },
  { id: "cam-8", name: "Cafeteria Overview", location: "Building C - Floor 1", rtsp_url: "rtsp://192.168.1.108:554/stream", status: "online", enabled: true, fps: 10, resolution: "1280x720", last_seen: new Date().toISOString(), created_at: "2024-06-12T08:00:00Z", updated_at: "2024-06-12T08:00:00Z" },
];

export const DEMO_INCIDENTS: Incident[] = [
  { id: "inc-1", camera_id: "cam-1", event_type: "Unauthorized Access", severity: "critical", status: "open", confidence: 0.95, bounding_box: { x: 120, y: 80, width: 200, height: 300 }, metadata: null, timestamp: new Date(Date.now() - 1800000).toISOString(), created_at: new Date(Date.now() - 1800000).toISOString(), updated_at: new Date(Date.now() - 1800000).toISOString() },
  { id: "inc-2", camera_id: "cam-3", event_type: "Suspicious Activity", severity: "high", status: "acknowledged", confidence: 0.88, bounding_box: null, metadata: null, timestamp: new Date(Date.now() - 3600000).toISOString(), created_at: new Date(Date.now() - 3600000).toISOString(), updated_at: new Date(Date.now() - 3600000).toISOString() },
  { id: "inc-3", camera_id: "cam-2", event_type: "Loitering Detected", severity: "medium", status: "open", confidence: 0.82, bounding_box: { x: 300, y: 200, width: 150, height: 250 }, metadata: null, timestamp: new Date(Date.now() - 7200000).toISOString(), created_at: new Date(Date.now() - 7200000).toISOString(), updated_at: new Date(Date.now() - 7200000).toISOString() },
  { id: "inc-4", camera_id: "cam-5", event_type: "Perimeter Breach", severity: "critical", status: "open", confidence: 0.97, bounding_box: { x: 50, y: 100, width: 400, height: 200 }, metadata: null, timestamp: new Date(Date.now() - 10800000).toISOString(), created_at: new Date(Date.now() - 10800000).toISOString(), updated_at: new Date(Date.now() - 10800000).toISOString() },
  { id: "inc-5", camera_id: "cam-1", event_type: "Tailgating Detected", severity: "high", status: "resolved", confidence: 0.91, bounding_box: { x: 80, y: 60, width: 180, height: 280 }, metadata: null, timestamp: new Date(Date.now() - 14400000).toISOString(), created_at: new Date(Date.now() - 14400000).toISOString(), updated_at: new Date(Date.now() - 14400000).toISOString() },
  { id: "inc-6", camera_id: "cam-6", event_type: "Object Left Behind", severity: "medium", status: "open", confidence: 0.78, bounding_box: { x: 200, y: 150, width: 100, height: 80 }, metadata: null, timestamp: new Date(Date.now() - 18000000).toISOString(), created_at: new Date(Date.now() - 18000000).toISOString(), updated_at: new Date(Date.now() - 18000000).toISOString() },
  { id: "inc-7", camera_id: "cam-8", event_type: "Fight Detected", severity: "critical", status: "acknowledged", confidence: 0.93, bounding_box: { x: 400, y: 250, width: 250, height: 350 }, metadata: null, timestamp: new Date(Date.now() - 21600000).toISOString(), created_at: new Date(Date.now() - 21600000).toISOString(), updated_at: new Date(Date.now() - 21600000).toISOString() },
  { id: "inc-8", camera_id: "cam-2", event_type: "Vehicle Intrusion", severity: "high", status: "resolved", confidence: 0.89, bounding_box: null, metadata: null, timestamp: new Date(Date.now() - 43200000).toISOString(), created_at: new Date(Date.now() - 43200000).toISOString(), updated_at: new Date(Date.now() - 43200000).toISOString() },
  { id: "inc-9", camera_id: "cam-4", event_type: "Camera Tampering", severity: "low", status: "false_positive", confidence: 0.65, bounding_box: null, metadata: null, timestamp: new Date(Date.now() - 86400000).toISOString(), created_at: new Date(Date.now() - 86400000).toISOString(), updated_at: new Date(Date.now() - 86400000).toISOString() },
  { id: "inc-10", camera_id: "cam-7", event_type: "Motion After Hours", severity: "medium", status: "resolved", confidence: 0.84, bounding_box: { x: 150, y: 100, width: 200, height: 180 }, metadata: null, timestamp: new Date(Date.now() - 129600000).toISOString(), created_at: new Date(Date.now() - 129600000).toISOString(), updated_at: new Date(Date.now() - 129600000).toISOString() },
];

export const DEMO_USERS: User[] = [
  { id: "user-1", email: "admin@vigilantai.com", first_name: "System", last_name: "Admin", is_active: true, roles: ["system_admin"], created_at: "2024-01-01T00:00:00Z", updated_at: "2024-01-01T00:00:00Z" },
  { id: "user-2", email: "sarah.chen@vigilantai.com", first_name: "Sarah", last_name: "Chen", is_active: true, roles: ["security_admin"], created_at: "2024-02-15T10:00:00Z", updated_at: "2024-02-15T10:00:00Z" },
  { id: "user-3", email: "mike.johnson@vigilantai.com", first_name: "Mike", last_name: "Johnson", is_active: true, roles: ["security_analyst"], created_at: "2024-03-01T09:00:00Z", updated_at: "2024-03-01T09:00:00Z" },
  { id: "user-4", email: "lisa.park@vigilantai.com", first_name: "Lisa", last_name: "Park", is_active: true, roles: ["operator"], created_at: "2024-04-10T08:00:00Z", updated_at: "2024-04-10T08:00:00Z" },
  { id: "user-5", email: "demo.viewer@vigilantai.com", first_name: "Demo", last_name: "Viewer", is_active: true, roles: ["viewer"], created_at: "2024-05-20T12:00:00Z", updated_at: "2024-05-20T12:00:00Z" },
];
