import apiClient from "@/lib/axios-client";
import type {
  Notification,
  SendNotificationRequest,
  NotificationFilters,
  PaginatedResponse,
} from "@/types";

export const notificationService = {
  async list(
    filters?: NotificationFilters
  ): Promise<PaginatedResponse<Notification>> {
    const response = await apiClient.get("/notifications", { params: filters });
    const raw = response.data;
    return { data: raw.notifications || raw.data || [], total: raw.total, page: raw.page, per_page: raw.per_page };
  },

  async getById(id: string): Promise<Notification> {
    const response = await apiClient.get(`/notifications/${id}`);
    return response.data.data;
  },

  async send(data: SendNotificationRequest): Promise<Notification> {
    const response = await apiClient.post("/notifications/send", data);
    return response.data.data;
  },

  async retryFailed(): Promise<Notification[]> {
    const response = await apiClient.post("/notifications/retry");
    return response.data.data;
  },
};
