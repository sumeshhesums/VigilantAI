export type NotificationChannel = "email" | "webhook";
export type NotificationStatus = "pending" | "sent" | "failed" | "retrying";

export interface Notification {
  id: string;
  incident_id: string;
  channel: NotificationChannel;
  recipient: string;
  status: NotificationStatus;
  attempts: number;
  response_code: number | null;
  error_message: string | null;
  created_at: string;
  sent_at: string | null;
}

export interface SendNotificationRequest {
  incident_id: string;
  channel: NotificationChannel;
  recipient: string;
}

export interface NotificationFilters {
  status?: NotificationStatus;
  channel?: NotificationChannel;
  incident_id?: string;
  page?: number;
  per_page?: number;
}
