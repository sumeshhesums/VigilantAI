import apiClient from "@/lib/axios-client";
import type {
  Camera,
  CreateCameraRequest,
  UpdateCameraRequest,
  CameraFilters,
  PaginatedResponse,
} from "@/types";
import { DEMO_CAMERAS } from "@/lib/demo-data";

export const cameraService = {
  async list(filters?: CameraFilters): Promise<PaginatedResponse<Camera>> {
    try {
      const response = await apiClient.get("/cameras", { params: filters });
      const raw = response.data;
      const data = raw.cameras || raw.data || [];
      if (data.length === 0) return { data: DEMO_CAMERAS, total: DEMO_CAMERAS.length, page: 1, per_page: 20 };
      return { data, total: raw.total ?? data.length, page: raw.page ?? 1, per_page: raw.per_page ?? 20 };
    } catch {
      return { data: DEMO_CAMERAS, total: DEMO_CAMERAS.length, page: 1, per_page: 20 };
    }
  },

  async getById(id: string): Promise<Camera> {
    try {
      const response = await apiClient.get(`/cameras/${id}`);
      return response.data;
    } catch {
      return DEMO_CAMERAS.find((c) => c.id === id) || DEMO_CAMERAS[0];
    }
  },

  async create(data: CreateCameraRequest): Promise<Camera> {
    try {
      const response = await apiClient.post("/cameras", data);
      return response.data;
    } catch {
      return { id: `cam-${Date.now()}`, name: data.name, location: data.location || null, rtsp_url: data.rtsp_url, status: "online", enabled: true, fps: data.fps || 15, resolution: data.resolution || "1920x1080", last_seen: null, created_at: new Date().toISOString(), updated_at: new Date().toISOString() };
    }
  },

  async update(id: string, data: UpdateCameraRequest): Promise<Camera> {
    try {
      const response = await apiClient.patch(`/cameras/${id}`, data);
      return response.data;
    } catch {
      const existing = DEMO_CAMERAS.find((c) => c.id === id) || DEMO_CAMERAS[0];
      return { ...existing, ...data, updated_at: new Date().toISOString() };
    }
  },

  async delete(id: string): Promise<void> {
    try { await apiClient.delete(`/cameras/${id}`); } catch { /* demo */ }
  },

  async enable(id: string): Promise<Camera> {
    try {
      const response = await apiClient.post(`/cameras/${id}/enable`);
      return response.data;
    } catch {
      const existing = DEMO_CAMERAS.find((c) => c.id === id) || DEMO_CAMERAS[0];
      return { ...existing, enabled: true, status: "online", updated_at: new Date().toISOString() };
    }
  },

  async disable(id: string): Promise<Camera> {
    try {
      const response = await apiClient.post(`/cameras/${id}/disable`);
      return response.data;
    } catch {
      const existing = DEMO_CAMERAS.find((c) => c.id === id) || DEMO_CAMERAS[0];
      return { ...existing, enabled: false, status: "offline", updated_at: new Date().toISOString() };
    }
  },
};
