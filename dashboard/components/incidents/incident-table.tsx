"use client";

import { useState } from "react";
import { useIncidents, useUpdateIncident } from "@/hooks/use-incidents";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Pagination } from "@/components/shared/pagination";
import { EmptyState } from "@/components/shared/empty-state";
import { ErrorState } from "@/components/shared/error-state";
import { TableSkeleton } from "@/components/shared/loading-skeleton";
import { SEVERITY_COLORS, INCIDENT_STATUS_COLORS } from "@/lib/constants";
import { timeAgo } from "@/lib/utils";
import { Search } from "lucide-react";
import type { IncidentStatus, IncidentSeverity } from "@/types";
import { IncidentDetailDrawer } from "./incident-detail-drawer";

export function IncidentTable() {
  const [page, setPage] = useState(1);
  const [perPage, setPerPage] = useState(20);
  const [search, setSearch] = useState("");
  const [severityFilter, setSeverityFilter] = useState<string>("");
  const [statusFilter, setStatusFilter] = useState<string>("");
  const [selectedIncidentId, setSelectedIncidentId] = useState<string | null>(null);

  const { data, isLoading, error, refetch } = useIncidents({
    page,
    per_page: perPage,
    severity: (severityFilter as IncidentSeverity) || undefined,
    status: (statusFilter as IncidentStatus) || undefined,
  });

  if (isLoading) return <TableSkeleton />;
  if (error) return <ErrorState onRetry={refetch} />;

  const incidents = data?.data || [];
  const filtered = search
    ? incidents.filter((i) => i.event_type.toLowerCase().includes(search.toLowerCase()))
    : incidents;

  return (
    <>
      <Card>
        <CardHeader>
          <CardTitle>Incidents</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="mb-4 flex flex-wrap gap-3">
            <div className="relative flex-1 min-w-[200px]">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input placeholder="Search incidents..." value={search} onChange={(e) => setSearch(e.target.value)} className="pl-9" />
            </div>
            <Select value={severityFilter} onValueChange={setSeverityFilter}>
              <SelectTrigger className="w-[140px]"><SelectValue placeholder="Severity" /></SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All severities</SelectItem>
                <SelectItem value="critical">Critical</SelectItem>
                <SelectItem value="high">High</SelectItem>
                <SelectItem value="medium">Medium</SelectItem>
                <SelectItem value="low">Low</SelectItem>
              </SelectContent>
            </Select>
            <Select value={statusFilter} onValueChange={setStatusFilter}>
              <SelectTrigger className="w-[140px]"><SelectValue placeholder="Status" /></SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All statuses</SelectItem>
                <SelectItem value="open">Open</SelectItem>
                <SelectItem value="acknowledged">Acknowledged</SelectItem>
                <SelectItem value="resolved">Resolved</SelectItem>
                <SelectItem value="false_positive">False Positive</SelectItem>
              </SelectContent>
            </Select>
          </div>

          {filtered.length === 0 ? (
            <EmptyState title="No incidents found" description="No incidents match your filters." />
          ) : (
            <>
              <div className="rounded-md border">
                <table className="w-full">
                  <thead>
                    <tr className="border-b bg-muted/50">
                      <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Event</th>
                      <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Severity</th>
                      <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Status</th>
                      <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Confidence</th>
                      <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Time</th>
                    </tr>
                  </thead>
                  <tbody>
                    {filtered.map((incident) => (
                      <tr
                        key={incident.id}
                        className="border-b transition-colors hover:bg-muted/50 cursor-pointer"
                        onClick={() => setSelectedIncidentId(incident.id)}
                      >
                        <td className="px-4 py-3 text-sm font-medium">{incident.event_type}</td>
                        <td className="px-4 py-3">
                          <Badge variant="outline" className={SEVERITY_COLORS[incident.severity]}>
                            {incident.severity}
                          </Badge>
                        </td>
                        <td className="px-4 py-3">
                          <Badge variant="outline" className={INCIDENT_STATUS_COLORS[incident.status]}>
                            {incident.status}
                          </Badge>
                        </td>
                        <td className="px-4 py-3 text-sm">{(incident.confidence * 100).toFixed(1)}%</td>
                        <td className="px-4 py-3 text-sm text-muted-foreground">{timeAgo(incident.created_at)}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
              <Pagination
                page={page}
                perPage={perPage}
                total={data?.total || 0}
                onPageChange={setPage}
                onPerPageChange={(v) => { setPerPage(v); setPage(1); }}
              />
            </>
          )}
        </CardContent>
      </Card>

      <IncidentDetailDrawer
        incidentId={selectedIncidentId}
        open={!!selectedIncidentId}
        onOpenChange={(open) => { if (!open) setSelectedIncidentId(null); }}
      />
    </>
  );
}
