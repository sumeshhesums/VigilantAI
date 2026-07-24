"use client";

import { useQuery } from "@tanstack/react-query";
import apiClient from "@/lib/axios-client";

export interface DashboardKPIs {
  active_cameras: number;
  online_cameras: number;
  offline_cameras: number;
  total_detections_24h: number;
  critical_alerts: number;
  open_incidents: number;
  avg_response_time_seconds: number;
  sla_compliance_percent: number;
  detection_trend: string;
}

export interface LiveStats {
  detections_per_minute: number;
  active_alerts: number;
  cameras_streaming: number;
  event_queue_depth: number;
  system_health: string;
  updated_at: string;
}

export interface AlertTrendSeries {
  timestamp: string;
  critical: number;
  high: number;
  medium: number;
  low: number;
}

export interface IncidentSummary {
  open: number;
  acknowledged: number;
  investigating: number;
  resolved: number;
  closed: number;
  avg_resolution_time_minutes: number;
  overdue_count: number;
}

export function useDashboardKPIs(siteId?: string) {
  return useQuery({
    queryKey: ["dashboard", "kpis", siteId],
    queryFn: async () => {
      const params = new URLSearchParams();
      if (siteId) params.set("site_id", siteId);
      const response = await apiClient.get(`/dashboard/kpis?${params.toString()}`);
      return response.data.data as DashboardKPIs;
    },
  });
}

export function useLiveStats() {
  return useQuery({
    queryKey: ["dashboard", "live-stats"],
    queryFn: async () => {
      const response = await apiClient.get("/dashboard/live-stats");
      return response.data.data as LiveStats;
    },
    refetchInterval: 60000,
  });
}

export function useAlertTrends(siteId?: string, from?: string, to?: string) {
  return useQuery({
    queryKey: ["dashboard", "alert-trends", siteId, from, to],
    queryFn: async () => {
      const params = new URLSearchParams();
      if (siteId) params.set("site_id", siteId);
      if (from) params.set("from", from);
      if (to) params.set("to", to);
      params.set("interval", "1h");
      const response = await apiClient.get(`/dashboard/alert-trends?${params.toString()}`);
      return response.data.data as { interval: string; series: AlertTrendSeries[] };
    },
  });
}

export function useIncidentSummary(siteId?: string) {
  return useQuery({
    queryKey: ["dashboard", "incidents-summary", siteId],
    queryFn: async () => {
      const params = new URLSearchParams();
      if (siteId) params.set("site_id", siteId);
      const response = await apiClient.get(`/dashboard/incidents-summary?${params.toString()}`);
      return response.data.data as IncidentSummary;
    },
  });
}
