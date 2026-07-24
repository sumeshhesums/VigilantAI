"use client";

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notificationService } from "@/services";
import type { NotificationFilters, SendNotificationRequest } from "@/types";

export function useNotifications(filters?: NotificationFilters) {
  return useQuery({
    queryKey: ["notifications", filters],
    queryFn: () => notificationService.list(filters),
  });
}

export function useNotification(id: string) {
  return useQuery({
    queryKey: ["notification", id],
    queryFn: () => notificationService.getById(id),
    enabled: !!id,
  });
}

export function useSendNotification() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: SendNotificationRequest) => notificationService.send(data),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["notifications"] }),
  });
}

export function useRetryNotifications() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => notificationService.retryFailed(),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["notifications"] }),
  });
}
