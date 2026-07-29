"use client";

import { useIncident, useUpdateIncident } from "@/hooks/use-incidents";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { SEVERITY_COLORS, INCIDENT_STATUS_COLORS } from "@/lib/constants";
import { formatDateTime } from "@/lib/utils";
import { useState } from "react";
import type { IncidentStatus } from "@/types";
import { Skeleton } from "@/components/shared/loading-skeleton";
import { toast } from "@/components/ui/toast";

interface IncidentDetailDrawerProps {
  incidentId: string | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function IncidentDetailDrawer({ incidentId, open, onOpenChange }: IncidentDetailDrawerProps) {
  const { data: incident, isLoading } = useIncident(incidentId || "");
  const updateMutation = useUpdateIncident();
  const [newStatus, setNewStatus] = useState<string>("");

  const handleStatusUpdate = () => {
    if (incidentId && newStatus) {
      updateMutation.mutate(
        { id: incidentId, data: { status: newStatus as IncidentStatus } },
        { onSuccess: () => { toast({ title: "Incident status updated", variant: "success" }); setNewStatus(""); } }
      );
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg">
        <DialogHeader>
          <DialogTitle>Incident Details</DialogTitle>
        </DialogHeader>
        {isLoading ? (
          <div className="space-y-3"><Skeleton className="h-20 w-full" /><Skeleton className="h-20 w-full" /></div>
        ) : !incident ? (
          <p className="text-sm text-muted-foreground">Incident not found.</p>
        ) : (
          <div className="space-y-4">
            <Card>
              <CardContent className="pt-6">
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Event Type</span>
                    <span className="text-sm font-medium">{incident.event_type}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Severity</span>
                    <Badge variant="outline" className={SEVERITY_COLORS[incident.severity]}>{incident.severity}</Badge>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Status</span>
                    <Badge variant="outline" className={INCIDENT_STATUS_COLORS[incident.status]}>{incident.status}</Badge>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Confidence</span>
                    <span className="text-sm">{(incident.confidence * 100).toFixed(1)}%</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Created</span>
                    <span className="text-sm">{formatDateTime(incident.created_at)}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Camera ID</span>
                    <span className="text-sm font-mono text-xs">{incident.camera_id}</span>
                  </div>
                </div>
              </CardContent>
            </Card>
            {incident.bounding_box && (
              <Card>
                <CardHeader><CardTitle className="text-sm">Bounding Box</CardTitle></CardHeader>
                <CardContent>
                  <div className="grid grid-cols-2 gap-2 text-sm">
                    <span className="text-muted-foreground">X: {incident.bounding_box.x}</span>
                    <span className="text-muted-foreground">Y: {incident.bounding_box.y}</span>
                    <span className="text-muted-foreground">Width: {incident.bounding_box.width}</span>
                    <span className="text-muted-foreground">Height: {incident.bounding_box.height}</span>
                  </div>
                </CardContent>
              </Card>
            )}
            <div className="flex gap-2">
              <Select value={newStatus} onValueChange={setNewStatus}>
                <SelectTrigger className="flex-1"><SelectValue placeholder="Change status" /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="open">Open</SelectItem>
                  <SelectItem value="acknowledged">Acknowledged</SelectItem>
                  <SelectItem value="resolved">Resolved</SelectItem>
                  <SelectItem value="false_positive">False Positive</SelectItem>
                </SelectContent>
              </Select>
              <Button onClick={handleStatusUpdate} disabled={!newStatus || updateMutation.isPending}>
                Update
              </Button>
            </div>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}
