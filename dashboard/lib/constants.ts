export const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080/api/v1";

export const SEVERITY_COLORS: Record<string, string> = {
  critical: "bg-red-500/20 text-red-400 border-red-500/30",
  high: "bg-orange-500/20 text-orange-400 border-orange-500/30",
  medium: "bg-yellow-500/20 text-yellow-400 border-yellow-500/30",
  low: "bg-blue-500/20 text-blue-400 border-blue-500/30",
};

export const STATUS_COLORS: Record<string, string> = {
  online: "bg-emerald-500/20 text-emerald-400 border-emerald-500/30",
  offline: "bg-red-500/20 text-red-400 border-red-500/30",
  maintenance: "bg-yellow-500/20 text-yellow-400 border-yellow-500/30",
};

export const INCIDENT_STATUS_COLORS: Record<string, string> = {
  open: "bg-blue-500/20 text-blue-400 border-blue-500/30",
  acknowledged: "bg-yellow-500/20 text-yellow-400 border-yellow-500/30",
  investigating: "bg-purple-500/20 text-purple-400 border-purple-500/30",
  resolved: "bg-emerald-500/20 text-emerald-400 border-emerald-500/30",
  false_positive: "bg-gray-500/20 text-gray-400 border-gray-500/30",
};

export const NOTIFICATION_STATUS_COLORS: Record<string, string> = {
  pending: "bg-yellow-500/20 text-yellow-400 border-yellow-500/30",
  sent: "bg-blue-500/20 text-blue-400 border-blue-500/30",
  retrying: "bg-orange-500/20 text-orange-400 border-orange-500/30",
  failed: "bg-red-500/20 text-red-400 border-red-500/30",
};

export const NOTIFICATION_CHANNEL_COLORS: Record<string, string> = {
  email: "bg-blue-500/20 text-blue-400 border-blue-500/30",
  webhook: "bg-purple-500/20 text-purple-400 border-purple-500/30",
};

export const PAGE_SIZES = [10, 20, 50, 100];
export const DEFAULT_PAGE_SIZE = 20;

export interface NavItem {
  title: string;
  href: string;
  icon: string;
  adminOnly?: boolean;
}

export const NAV_ITEMS: NavItem[] = [
  { title: "Dashboard", href: "/", icon: "LayoutDashboard" },
  { title: "Cameras", href: "/cameras", icon: "Camera" },
  { title: "Incidents", href: "/incidents", icon: "AlertTriangle" },
  { title: "Evidence", href: "/evidence", icon: "FileVideo" },
  { title: "Notifications", href: "/notifications", icon: "Bell" },
  { title: "Users", href: "/users", icon: "Users", adminOnly: true },
  { title: "Roles", href: "/roles", icon: "Shield", adminOnly: true },
  { title: "Analytics", href: "/analytics", icon: "BarChart3" },
  { title: "System Health", href: "/system-health", icon: "Activity" },
  { title: "Settings", href: "/settings", icon: "Settings" },
];
