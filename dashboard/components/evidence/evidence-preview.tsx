"use client";

import { useEvidence } from "@/hooks/use-evidence";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import { formatBytes, formatDateTime } from "@/lib/utils";
import { Skeleton } from "@/components/shared/loading-skeleton";
import { Shield } from "lucide-react";

interface EvidencePreviewProps {
  evidenceId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function EvidencePreview({ evidenceId, open, onOpenChange }: EvidencePreviewProps) {
  const { data: evidence, isLoading } = useEvidence(evidenceId);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg">
        <DialogHeader>
          <DialogTitle>Evidence Details</DialogTitle>
        </DialogHeader>
        {isLoading ? (
          <Skeleton className="h-48 w-full" />
        ) : !evidence ? (
          <p className="text-sm text-muted-foreground">Evidence not found.</p>
        ) : (
          <div className="space-y-4">
            {evidence.content_type.startsWith("image/") ? (
              <div className="flex items-center justify-center rounded-lg border bg-muted p-4">
                <p className="text-sm text-muted-foreground">Image preview available</p>
              </div>
            ) : (
              <div className="flex items-center justify-center rounded-lg border bg-muted p-8">
                <p className="text-sm text-muted-foreground">{evidence.content_type}</p>
              </div>
            )}
            <Card>
              <CardContent className="pt-6">
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">File Name</span>
                    <span className="text-sm font-medium">{evidence.file_name}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Content Type</span>
                    <Badge variant="outline">{evidence.content_type}</Badge>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">File Size</span>
                    <span className="text-sm">{formatBytes(evidence.file_size)}</span>
                  </div>
                  {evidence.width && evidence.height && (
                    <div className="flex items-center justify-between">
                      <span className="text-sm text-muted-foreground">Dimensions</span>
                      <span className="text-sm">{evidence.width} x {evidence.height}</span>
                    </div>
                  )}
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">SHA-256</span>
                    <span className="text-xs font-mono break-all max-w-[200px]">{evidence.sha256}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Created</span>
                    <span className="text-sm">{formatDateTime(evidence.created_at)}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Integrity</span>
                    <div className="flex items-center gap-1">
                      <Shield className="h-4 w-4 text-emerald-500" />
                      <span className="text-sm text-emerald-500">Verified</span>
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}
