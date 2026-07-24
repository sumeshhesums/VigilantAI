"use client";

import { Bell } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useNotifications } from "@/hooks/use-notifications";

export function NotificationBell() {
  const { data } = useNotifications({ status: "failed", per_page: 5 });
  const failedCount = data?.total || 0;

  return (
    <Button variant="ghost" size="icon" className="relative">
      <Bell className="h-5 w-5" />
      {failedCount > 0 && (
        <span className="absolute -right-1 -top-1 flex h-4 w-4 items-center justify-center rounded-full bg-destructive text-[10px] font-medium text-destructive-foreground">
          {failedCount > 9 ? "9+" : failedCount}
        </span>
      )}
    </Button>
  );
}
