"use client";

import { NotificationTable } from "@/components/notifications/notification-table";

export default function NotificationsPage() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Notifications</h1>
        <p className="text-muted-foreground">Notification history and delivery status</p>
      </div>
      <NotificationTable />
    </div>
  );
}
