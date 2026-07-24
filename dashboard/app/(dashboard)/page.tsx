"use client";

import { StatCard } from "@/components/dashboard/stat-card";
import { RecentIncidents } from "@/components/dashboard/recent-incidents";
import { RecentNotifications } from "@/components/dashboard/recent-notifications";
import { HealthStatus } from "@/components/dashboard/health-status";
import { AlertsChart } from "@/components/dashboard/alerts-chart";
import { useDashboardKPIs } from "@/hooks/use-dashboard";
import { DashboardSkeleton } from "@/components/shared/loading-skeleton";
import { ErrorState } from "@/components/shared/error-state";
import {
  Camera,
  Wifi,
  AlertTriangle,
  ShieldAlert,
  FileVideo,
  Bell,
} from "lucide-react";

export default function DashboardPage() {
  const { data: kpis, isLoading, error, refetch } = useDashboardKPIs();

  if (isLoading) return <DashboardSkeleton />;
  if (error) return <ErrorState onRetry={refetch} />;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Dashboard</h1>
        <p className="text-muted-foreground">Security operations overview</p>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-6">
        <StatCard
          title="Total Cameras"
          value={kpis?.active_cameras || 0}
          icon={Camera}
        />
        <StatCard
          title="Online"
          value={kpis?.online_cameras || 0}
          icon={Wifi}
          description={`${kpis?.offline_cameras || 0} offline`}
        />
        <StatCard
          title="Open Incidents"
          value={kpis?.open_incidents || 0}
          icon={AlertTriangle}
        />
        <StatCard
          title="Critical Alerts"
          value={kpis?.critical_alerts || 0}
          icon={ShieldAlert}
        />
        <StatCard
          title="Detections (24h)"
          value={kpis?.total_detections_24h || 0}
          icon={FileVideo}
          trend={kpis?.detection_trend}
          trendUp={kpis?.detection_trend?.startsWith("+")}
        />
        <StatCard
          title="SLA Compliance"
          value={`${kpis?.sla_compliance_percent || 0}%`}
          icon={Bell}
        />
      </div>

      <div className="grid gap-6 lg:grid-cols-7">
        <div className="lg:col-span-4">
          <AlertsChart />
        </div>
        <div className="lg:col-span-3">
          <HealthStatus />
        </div>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        <RecentIncidents />
        <RecentNotifications />
      </div>
    </div>
  );
}
