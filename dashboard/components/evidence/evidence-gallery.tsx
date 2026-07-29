"use client";

import { useState } from "react";
import { useEvidenceByIncident, useDeleteEvidence } from "@/hooks/use-evidence";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Pagination } from "@/components/shared/pagination";
import { EmptyState } from "@/components/shared/empty-state";
import { ErrorState } from "@/components/shared/error-state";
import { TableSkeleton } from "@/components/shared/loading-skeleton";
import { ConfirmDialog } from "@/components/shared/confirm-dialog";
import { formatBytes, formatDateTime } from "@/lib/utils";
import { Search, Download, Trash2, Shield, FileVideo, Image, FileText } from "lucide-react";
import { EvidencePreview } from "./evidence-preview";
import { toast } from "@/components/ui/toast";

const getEvidenceIcon = (contentType: string) => {
  if (contentType.startsWith("video/")) return FileVideo;
  if (contentType.startsWith("image/")) return Image;
  return FileText;
};

interface EvidenceGalleryProps {
  incidentId?: string;
}

export function EvidenceGallery({ incidentId }: EvidenceGalleryProps) {
  const [page, setPage] = useState(1);
  const [perPage, setPerPage] = useState(20);
  const [search, setSearch] = useState("");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [deleteId, setDeleteId] = useState<string | null>(null);

  const { data, isLoading, error, refetch } = useEvidenceByIncident(
    incidentId || "",
    { page, per_page: perPage }
  );

  const deleteMutation = useDeleteEvidence();

  if (isLoading) return <TableSkeleton />;
  if (error) return <ErrorState onRetry={refetch} />;
  if (!incidentId) {
    return <EmptyState title="Select an incident" description="Choose an incident to view its evidence." />;
  }

  const evidence = data?.data || [];
  const filtered = search
    ? evidence.filter((e) => e.file_name.toLowerCase().includes(search.toLowerCase()))
    : evidence;

  return (
    <>
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>Evidence ({data?.total || 0})</CardTitle>
          </div>
        </CardHeader>
        <CardContent>
          <div className="mb-4">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                placeholder="Search evidence..."
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                className="pl-9"
              />
            </div>
          </div>

          {filtered.length === 0 ? (
            <EmptyState title="No evidence found" description="No evidence items for this incident." />
          ) : (
            <>
              <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
                {filtered.map((item) => {
                  const Icon = getEvidenceIcon(item.content_type);
                  return (
                    <div
                      key={item.id}
                      className="group cursor-pointer rounded-lg border p-4 transition-colors hover:bg-muted/50"
                      onClick={() => setSelectedId(item.id)}
                    >
                      <div className="flex items-start justify-between">
                        <div className="flex items-center gap-2">
                          <Icon className="h-5 w-5 text-muted-foreground" />
                          <span className="text-sm font-medium truncate max-w-[150px]">{item.file_name}</span>
                        </div>
                        <div className="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                          <Button variant="ghost" size="icon" className="h-7 w-7" onClick={(e) => { e.stopPropagation(); }}>
                            <Download className="h-3 w-3" />
                          </Button>
                          <Button variant="ghost" size="icon" className="h-7 w-7" onClick={(e) => { e.stopPropagation(); setDeleteId(item.id); }}>
                            <Trash2 className="h-3 w-3 text-destructive" />
                          </Button>
                        </div>
                      </div>
                      <div className="mt-3 space-y-1">
                        <p className="text-xs text-muted-foreground">{formatBytes(item.file_size)}</p>
                        <div className="flex items-center gap-1">
                          <Shield className="h-3 w-3 text-emerald-500" />
                          <span className="text-xs text-muted-foreground font-mono truncate max-w-[200px]">{item.sha256}</span>
                        </div>
                        <p className="text-xs text-muted-foreground">{formatDateTime(item.created_at)}</p>
                      </div>
                    </div>
                  );
                })}
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

      {selectedId && (
        <EvidencePreview evidenceId={selectedId} open={!!selectedId} onOpenChange={(o) => { if (!o) setSelectedId(null); }} />
      )}

      <ConfirmDialog
        open={!!deleteId}
        onOpenChange={() => setDeleteId(null)}
        title="Delete Evidence"
        description="Are you sure you want to delete this evidence? This action cannot be undone."
        confirmLabel="Delete"
        onConfirm={() => {
          if (deleteId) {
            deleteMutation.mutate(deleteId, { onSuccess: () => { toast({ title: "Evidence deleted", variant: "success" }); setDeleteId(null); } });
          }
        }}
        loading={deleteMutation.isPending}
      />
    </>
  );
}
