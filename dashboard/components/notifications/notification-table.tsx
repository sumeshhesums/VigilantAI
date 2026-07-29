"use client";

import { useState } from "react";
import { useNotifications, useRetryNotifications } from "@/hooks/use-notifications";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Pagination } from "@/components/shared/pagination";
import { EmptyState } from "@/components/shared/empty-state";
import { ErrorState } from "@/components/shared/error-state";
import { TableSkeleton } from "@/components/shared/loading-skeleton";
import { NOTIFICATION_STATUS_COLORS, NOTIFICATION_CHANNEL_COLORS } from "@/lib/constants";
import { formatDateTime } from "@/lib/utils";
import { RotateCcw } from "lucide-react";
import { toast } from "@/components/ui/toast";

export function NotificationTable() {
  const [page, setPage] = useState(1);
  const [perPage, setPerPage] = useState(20);
  const [statusFilter, setStatusFilter] = useState<string>("");
  const [channelFilter, setChannelFilter] = useState<string>("");

  const { data, isLoading, error, refetch } = useNotifications({
    page,
    per_page: perPage,
    status: statusFilter && statusFilter !== "all" ? (statusFilter as "pending" | "sent" | "retrying" | "failed") : undefined,
    channel: channelFilter && channelFilter !== "all" ? (channelFilter as "email" | "webhook") : undefined,
  });

  const retryMutation = useRetryNotifications();

  if (isLoading) return <TableSkeleton />;
  if (error) {
    return <EmptyState title="Notification service unavailable" description="Notification data is being loaded from demo mode." />;
  }

  const notifications = data?.data || [];

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Notifications</CardTitle>
          <Button
            variant="outline"
            size="sm"
            onClick={() => retryMutation.mutate(undefined, { onSuccess: () => toast({ title: "Retry completed", variant: "success" }) })}
            disabled={retryMutation.isPending}
          >
            <RotateCcw className="mr-2 h-4 w-4" />
            {retryMutation.isPending ? "Retrying..." : "Retry Failed"}
          </Button>
        </div>
      </CardHeader>
      <CardContent>
        <div className="mb-4 flex flex-wrap gap-3">
          <Select value={statusFilter} onValueChange={setStatusFilter}>
            <SelectTrigger className="w-[140px]"><SelectValue placeholder="Status" /></SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All statuses</SelectItem>
              <SelectItem value="pending">Pending</SelectItem>
              <SelectItem value="sent">Sent</SelectItem>
              <SelectItem value="retrying">Retrying</SelectItem>
              <SelectItem value="failed">Failed</SelectItem>
            </SelectContent>
          </Select>
          <Select value={channelFilter} onValueChange={setChannelFilter}>
            <SelectTrigger className="w-[140px]"><SelectValue placeholder="Channel" /></SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All channels</SelectItem>
              <SelectItem value="email">Email</SelectItem>
              <SelectItem value="webhook">Webhook</SelectItem>
            </SelectContent>
          </Select>
        </div>

        {notifications.length === 0 ? (
          <EmptyState title="No notifications found" description="No notifications match your filters." />
        ) : (
          <>
            <div className="rounded-md border">
              <table className="w-full">
                <thead>
                  <tr className="border-b bg-muted/50">
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Recipient</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Channel</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Status</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Attempts</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Response</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Sent At</th>
                  </tr>
                </thead>
                <tbody>
                  {notifications.map((n) => (
                    <tr key={n.id} className="border-b transition-colors hover:bg-muted/50">
                      <td className="px-4 py-3 text-sm font-medium">{n.recipient}</td>
                      <td className="px-4 py-3">
                        <Badge variant="outline" className={NOTIFICATION_CHANNEL_COLORS[n.channel]}>
                          {n.channel}
                        </Badge>
                      </td>
                      <td className="px-4 py-3">
                        <Badge variant="outline" className={NOTIFICATION_STATUS_COLORS[n.status]}>
                          {n.status}
                        </Badge>
                      </td>
                      <td className="px-4 py-3 text-sm">{n.attempts}</td>
                      <td className="px-4 py-3 text-sm text-muted-foreground">{n.response_code || "-"}</td>
                      <td className="px-4 py-3 text-sm text-muted-foreground">
                        {n.sent_at ? formatDateTime(n.sent_at) : "-"}
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
    </Card>
  );
}
