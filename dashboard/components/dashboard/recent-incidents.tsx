"use client";

import { useIncidents } from "@/hooks/use-incidents";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { SEVERITY_COLORS, INCIDENT_STATUS_COLORS } from "@/lib/constants";
import { timeAgo } from "@/lib/utils";
import Link from "next/link";
import { Button } from "@/components/ui/button";
import { ArrowRight } from "lucide-react";
import { EmptyState } from "@/components/shared/empty-state";
import { Skeleton } from "@/components/shared/loading-skeleton";

export function RecentIncidents() {
  const { data, isLoading } = useIncidents({ per_page: 5 });

  if (isLoading) {
    return (
      <Card>
        <CardHeader><Skeleton className="h-5 w-40" /></CardHeader>
        <CardContent className="space-y-3">
          {Array.from({ length: 5 }).map((_, i) => <Skeleton key={i} className="h-12 w-full" />)}
        </CardContent>
      </Card>
    );
  }

  const incidents = data?.data || [];

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <CardTitle className="text-base">Recent Incidents</CardTitle>
        <Button variant="ghost" size="sm" asChild>
          <Link href="/incidents">View all <ArrowRight className="ml-1 h-4 w-4" /></Link>
        </Button>
      </CardHeader>
      <CardContent>
        {incidents.length === 0 ? (
          <EmptyState title="No incidents" description="No incidents recorded yet." />
        ) : (
          <div className="space-y-3">
            {incidents.map((incident) => (
              <div key={incident.id} className="flex items-center justify-between rounded-lg border p-3">
                <div className="space-y-1">
                  <p className="text-sm font-medium">{incident.event_type}</p>
                  <p className="text-xs text-muted-foreground">{timeAgo(incident.created_at)}</p>
                </div>
                <div className="flex items-center gap-2">
                  <Badge variant="outline" className={SEVERITY_COLORS[incident.severity]}>
                    {incident.severity}
                  </Badge>
                  <Badge variant="outline" className={INCIDENT_STATUS_COLORS[incident.status]}>
                    {incident.status}
                  </Badge>
                </div>
              </div>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
