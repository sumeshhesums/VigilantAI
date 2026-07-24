"use client";

import { useHealth } from "@/hooks/use-health";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/shared/loading-skeleton";
import { ErrorState } from "@/components/shared/error-state";
import { CheckCircle, XCircle, Clock } from "lucide-react";
import { formatDateTime } from "@/lib/utils";

export default function SystemHealthPage() {
  const { data, isLoading, error, refetch } = useHealth();

  if (isLoading) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">System Health</h1>
          <p className="text-muted-foreground">Monitor platform service health</p>
        </div>
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {Array.from({ length: 6 }).map((_, i) => (
            <Skeleton key={i} className="h-[120px] rounded-xl" />
          ))}
        </div>
      </div>
    );
  }

  if (error) return <ErrorState onRetry={refetch} />;

  const services = data?.services || {} as Record<string, { status: string; latency_ms?: number; active_streams?: number; used_gb?: number }>;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">System Health</h1>
        <p className="text-muted-foreground">Monitor platform service health</p>
      </div>

      <div className="flex items-center gap-4">
        <Badge
          variant="outline"
          className={
            data?.status === "healthy"
              ? "bg-emerald-500/20 text-emerald-400 border-emerald-500/30"
              : "bg-red-500/20 text-red-400 border-red-500/30"
          }
        >
          {data?.status === "healthy" ? (
            <CheckCircle className="mr-1 h-3 w-3" />
          ) : (
            <XCircle className="mr-1 h-3 w-3" />
          )}
          {data?.status || "Unknown"}
        </Badge>
        {data?.version && <span className="text-sm text-muted-foreground">Version: {data.version}</span>}
        {data?.uptime_seconds !== undefined && (
          <span className="text-sm text-muted-foreground flex items-center gap-1">
            <Clock className="h-3 w-3" />
            Uptime: {Math.floor(data.uptime_seconds / 86400)}d {Math.floor((data.uptime_seconds % 86400) / 3600)}h
          </span>
        )}
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {Object.entries(services).map(([name, svc]) => (
          <Card key={name}>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium capitalize">{name.replace(/_/g, " ")}</CardTitle>
              {svc.status === "healthy" ? (
                <CheckCircle className="h-4 w-4 text-emerald-500" />
              ) : (
                <XCircle className="h-4 w-4 text-red-500" />
              )}
            </CardHeader>
            <CardContent>
              <Badge
                variant="outline"
                className={
                  svc.status === "healthy"
                    ? "bg-emerald-500/20 text-emerald-400 border-emerald-500/30"
                    : "bg-red-500/20 text-red-400 border-red-500/30"
                }
              >
                {svc.status}
              </Badge>
              <div className="mt-3 space-y-1">
                {svc.latency_ms !== undefined && (
                  <p className="text-xs text-muted-foreground">Latency: {svc.latency_ms}ms</p>
                )}
                {svc.active_streams !== undefined && (
                  <p className="text-xs text-muted-foreground">Active streams: {svc.active_streams}</p>
                )}
                {svc.used_gb !== undefined && (
                  <p className="text-xs text-muted-foreground">Storage: {svc.used_gb} GB</p>
                )}
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}
