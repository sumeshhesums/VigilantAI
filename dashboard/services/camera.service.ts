import apiClient from "@/lib/axios-client";
import type {
  Camera,
  CreateCameraRequest,
  UpdateCameraRequest,
  CameraFilters,
  PaginatedResponse,
} from "@/types";

export const cameraService = {
  async list(filters?: CameraFilters): Promise<PaginatedResponse<Camera>> {
    const response = await apiClient.get("/cameras", { params: filters });
    const raw = response.data;
    return { data: raw.cameras || raw.data || [], total: raw.total, page: raw.page, per_page: raw.per_page };
  },

  async getById(id: string): Promise<Camera> {
    const response = await apiClient.get(`/cameras/${id}`);
    return response.data.data;
  },

  async create(data: CreateCameraRequest): Promise<Camera> {
    const response = await apiClient.post("/cameras", data);
    return response.data.data;
  },

  async update(id: string, data: UpdateCameraRequest): Promise<Camera> {
    const response = await apiClient.patch(`/cameras/${id}`, data);
    return response.data.data;
  },

  async delete(id: string): Promise<void> {
    await apiClient.delete(`/cameras/${id}`);
  },

  async enable(id: string): Promise<Camera> {
    const response = await apiClient.post(`/cameras/${id}/enable`);
    return response.data.data;
  },

  async disable(id: string): Promise<Camera> {
    const response = await apiClient.post(`/cameras/${id}/disable`);
    return response.data.data;
  },
};
