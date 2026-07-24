"use client";

import { useNotifications } from "@/hooks/use-notifications";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { NOTIFICATION_STATUS_COLORS, NOTIFICATION_CHANNEL_COLORS } from "@/lib/constants";
import { timeAgo } from "@/lib/utils";
import Link from "next/link";
import { Button } from "@/components/ui/button";
import { ArrowRight } from "lucide-react";
import { EmptyState } from "@/components/shared/empty-state";
import { Skeleton } from "@/components/shared/loading-skeleton";

export function RecentNotifications() {
  const { data, isLoading } = useNotifications({ per_page: 5 });

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

  const notifications = data?.data || [];

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <CardTitle className="text-base">Recent Notifications</CardTitle>
        <Button variant="ghost" size="sm" asChild>
          <Link href="/notifications">View all <ArrowRight className="ml-1 h-4 w-4" /></Link>
        </Button>
      </CardHeader>
      <CardContent>
        {notifications.length === 0 ? (
          <EmptyState title="No notifications" description="No notifications sent yet." />
        ) : (
          <div className="space-y-3">
            {notifications.map((n) => (
              <div key={n.id} className="flex items-center justify-between rounded-lg border p-3">
                <div className="space-y-1">
                  <p className="text-sm font-medium">{n.recipient}</p>
                  <p className="text-xs text-muted-foreground">{timeAgo(n.created_at)}</p>
                </div>
                <div className="flex items-center gap-2">
                  <Badge variant="outline" className={NOTIFICATION_CHANNEL_COLORS[n.channel]}>
                    {n.channel}
                  </Badge>
                  <Badge variant="outline" className={NOTIFICATION_STATUS_COLORS[n.status]}>
                    {n.status}
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
