"use client";

import { useState } from "react";
import { EvidenceGallery } from "@/components/evidence/evidence-gallery";
import { useIncidents } from "@/hooks/use-incidents";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Label } from "@/components/ui/label";

export default function EvidencePage() {
  const [selectedIncidentId, setSelectedIncidentId] = useState<string>("");
  const { data: incidentsData } = useIncidents({ per_page: 100 });

  const incidents = incidentsData?.data || [];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Evidence</h1>
        <p className="text-muted-foreground">View and manage forensic evidence</p>
      </div>

      <div className="space-y-2">
        <Label>Select Incident</Label>
        <Select value={selectedIncidentId} onValueChange={setSelectedIncidentId}>
          <SelectTrigger className="w-full max-w-md">
            <SelectValue placeholder="Choose an incident to view evidence" />
          </SelectTrigger>
          <SelectContent>
            {incidents.map((inc) => (
              <SelectItem key={inc.id} value={inc.id}>
                {inc.event_type} - {inc.severity} ({inc.status})
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>

      <EvidenceGallery incidentId={selectedIncidentId} />
    </div>
  );
}
