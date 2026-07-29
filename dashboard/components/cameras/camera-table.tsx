"use client";

import { useState } from "react";
import { useCameras, useDeleteCamera, useEnableCamera, useDisableCamera } from "@/hooks/use-cameras";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Pagination } from "@/components/shared/pagination";
import { EmptyState } from "@/components/shared/empty-state";
import { ErrorState } from "@/components/shared/error-state";
import { TableSkeleton } from "@/components/shared/loading-skeleton";
import { ConfirmDialog } from "@/components/shared/confirm-dialog";
import { SEVERITY_COLORS, STATUS_COLORS } from "@/lib/constants";
import { timeAgo } from "@/lib/utils";
import { Search, Plus, Trash2, Power, PowerOff } from "lucide-react";
import { toast } from "@/components/ui/toast";

interface CameraTableProps {
  onEdit?: (id: string) => void;
  onCreate?: () => void;
}

export function CameraTable({ onEdit, onCreate }: CameraTableProps) {
  const [page, setPage] = useState(1);
  const [perPage, setPerPage] = useState(20);
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<string>("");

  const { data, isLoading, error, refetch } = useCameras({
    page,
    per_page: perPage,
    status: statusFilter && statusFilter !== "all" ? statusFilter : undefined,
  });

  const deleteMutation = useDeleteCamera();
  const enableMutation = useEnableCamera();
  const disableMutation = useDisableCamera();
  const [deleteId, setDeleteId] = useState<string | null>(null);

  if (isLoading) return <TableSkeleton />;
  if (error) return <ErrorState onRetry={refetch} />;

  const cameras = data?.data || [];
  const filtered = search
    ? cameras.filter((c) => c.name.toLowerCase().includes(search.toLowerCase()))
    : cameras;

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Cameras</CardTitle>
          {onCreate && (
            <Button onClick={onCreate}>
              <Plus className="mr-2 h-4 w-4" /> Add Camera
            </Button>
          )}
        </div>
      </CardHeader>
      <CardContent>
        <div className="mb-4 flex flex-wrap gap-3">
          <div className="relative flex-1 min-w-[200px]">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              placeholder="Search cameras..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="pl-9"
            />
          </div>
          <Select value={statusFilter} onValueChange={setStatusFilter}>
            <SelectTrigger className="w-[150px]">
              <SelectValue placeholder="All statuses" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All statuses</SelectItem>
              <SelectItem value="online">Online</SelectItem>
              <SelectItem value="offline">Offline</SelectItem>
              <SelectItem value="maintenance">Maintenance</SelectItem>
            </SelectContent>
          </Select>
        </div>

        {filtered.length === 0 ? (
          <EmptyState title="No cameras found" description="Add a camera to get started." action={onCreate ? { label: "Add Camera", onClick: onCreate } : undefined} />
        ) : (
          <>
            <div className="rounded-md border">
              <table className="w-full">
                <thead>
                  <tr className="border-b bg-muted/50">
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Name</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Status</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Location</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">FPS</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Last Seen</th>
                    <th className="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {filtered.map((camera) => (
                    <tr key={camera.id} className="border-b transition-colors hover:bg-muted/50">
                      <td className="px-4 py-3">
                        <button onClick={() => onEdit?.(camera.id)} className="text-sm font-medium hover:underline">
                          {camera.name}
                        </button>
                      </td>
                      <td className="px-4 py-3">
                        <Badge variant="outline" className={STATUS_COLORS[camera.status] || ""}>
                          {camera.status}
                        </Badge>
                      </td>
                      <td className="px-4 py-3 text-sm text-muted-foreground">{camera.location || "-"}</td>
                      <td className="px-4 py-3 text-sm">{camera.fps || "-"}</td>
                      <td className="px-4 py-3 text-sm text-muted-foreground">
                        {camera.last_seen ? timeAgo(camera.last_seen) : "Never"}
                      </td>
                      <td className="px-4 py-3 text-right">
                        <div className="flex items-center justify-end gap-1">
                          <Button
                            variant="ghost"
                            size="icon"
                            className="h-8 w-8"
                            onClick={() => camera.enabled ? disableMutation.mutate(camera.id, { onSuccess: () => toast({ title: "Camera disabled", variant: "success" }) }) : enableMutation.mutate(camera.id, { onSuccess: () => toast({ title: "Camera enabled", variant: "success" }) })}
                          >
                            {camera.enabled ? <PowerOff className="h-4 w-4" /> : <Power className="h-4 w-4" />}
                          </Button>
                          <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => setDeleteId(camera.id)}>
                            <Trash2 className="h-4 w-4 text-destructive" />
                          </Button>
                        </div>
                      </td>
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

      <ConfirmDialog
        open={!!deleteId}
        onOpenChange={() => setDeleteId(null)}
        title="Delete Camera"
        description="Are you sure you want to delete this camera? This action cannot be undone."
        confirmLabel="Delete"
        onConfirm={() => {
          if (deleteId) {
            deleteMutation.mutate(deleteId, { onSuccess: () => { toast({ title: "Camera deleted", variant: "success" }); setDeleteId(null); } });
          }
        }}
        loading={deleteMutation.isPending}
      />
    </Card>
  );
}
