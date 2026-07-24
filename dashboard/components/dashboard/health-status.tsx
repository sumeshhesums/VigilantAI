"use client";

import { useHealth } from "@/hooks/use-health";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/shared/loading-skeleton";
import { CheckCircle, XCircle } from "lucide-react";

export function HealthStatus() {
  const { data, isLoading } = useHealth();

  if (isLoading) {
    return (
      <Card>
        <CardHeader><Skeleton className="h-5 w-32" /></CardHeader>
        <CardContent className="space-y-2">
          {Array.from({ length: 5 }).map((_, i) => <Skeleton key={i} className="h-8 w-full" />)}
        </CardContent>
      </Card>
    );
  }

  const services = data?.services || {} as Record<string, { status: string; latency_ms?: number }>;

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base">System Health</CardTitle>
      </CardHeader>
      <CardContent className="space-y-2">
        {Object.entries(services).map(([name, svc]) => (
          <div key={name} className="flex items-center justify-between rounded-lg border p-3">
            <div className="flex items-center gap-2">
              {svc.status === "healthy" ? (
                <CheckCircle className="h-4 w-4 text-emerald-500" />
              ) : (
                <XCircle className="h-4 w-4 text-red-500" />
              )}
              <span className="text-sm font-medium capitalize">{name.replace(/_/g, " ")}</span>
            </div>
            <div className="flex items-center gap-2">
              {svc.latency_ms !== undefined && (
                <span className="text-xs text-muted-foreground">{svc.latency_ms}ms</span>
              )}
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
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
