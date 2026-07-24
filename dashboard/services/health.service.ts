import apiClient from "@/lib/axios-client";
import type { HealthStatus } from "@/types";

export const healthService = {
  async getHealth(): Promise<HealthStatus> {
    const response = await apiClient.get("/health");
    return response.data.data;
  },
};
