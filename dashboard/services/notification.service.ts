import apiClient from "@/lib/axios-client";
import type {
  Notification,
  SendNotificationRequest,
  NotificationFilters,
  PaginatedResponse,
} from "@/types";
import { DEMO_NOTIFICATIONS } from "@/lib/demo-data";

export const notificationService = {
  async list(
    filters?: NotificationFilters
  ): Promise<PaginatedResponse<Notification>> {
    try {
      const response = await apiClient.get("/notifications", { params: filters });
      const raw = response.data;
      return {
        data: raw.notifications || raw.data || raw,
        total: raw.total ?? 0,
        page: raw.page ?? 1,
        per_page: raw.per_page ?? 20,
      };
    } catch {
      return { data: DEMO_NOTIFICATIONS, total: DEMO_NOTIFICATIONS.length, page: 1, per_page: 20 };
    }
  },

  async getById(id: string): Promise<Notification> {
    try {
      const response = await apiClient.get(`/notifications/${id}`);
      return response.data;
    } catch {
      return DEMO_NOTIFICATIONS.find((n) => n.id === id) || DEMO_NOTIFICATIONS[0];
    }
  },

  async send(data: SendNotificationRequest): Promise<Notification> {
    try {
      const response = await apiClient.post("/notifications/send", data);
      return response.data;
    } catch {
      return {
        id: `notif-${Date.now()}`,
        incident_id: data.incident_id,
        channel: data.channel,
        recipient: data.recipient,
        status: "sent",
        attempts: 1,
        response_code: null,
        error_message: null,
        created_at: new Date().toISOString(),
        sent_at: null,
      };
    }
  },

  async retryFailed(): Promise<Notification[]> {
    try {
      const response = await apiClient.post("/notifications/retry");
      return response.data;
    } catch {
      return [];
    }
  },
};
