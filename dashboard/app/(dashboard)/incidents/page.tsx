"use client";

import { IncidentTable } from "@/components/incidents/incident-table";

export default function IncidentsPage() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Incidents</h1>
        <p className="text-muted-foreground">Monitor and manage security incidents</p>
      </div>
      <IncidentTable />
    </div>
  );
}
