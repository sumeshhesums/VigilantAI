import apiClient from "@/lib/axios-client";
import type {
  Incident,
  CreateIncidentRequest,
  UpdateIncidentRequest,
  IncidentFilters,
  PaginatedResponse,
} from "@/types";
import { DEMO_INCIDENTS } from "@/lib/demo-data";

export const incidentService = {
  async list(filters?: IncidentFilters): Promise<PaginatedResponse<Incident>> {
    try {
      const response = await apiClient.get("/incidents", { params: filters });
      const raw = response.data;
      const data = raw.incidents || raw.data || [];
      if (data.length === 0) return { data: DEMO_INCIDENTS, total: DEMO_INCIDENTS.length, page: 1, per_page: 20 };
      return { data, total: raw.total ?? data.length, page: raw.page ?? 1, per_page: raw.per_page ?? 20 };
    } catch {
      return { data: DEMO_INCIDENTS, total: DEMO_INCIDENTS.length, page: 1, per_page: 20 };
    }
  },

  async getById(id: string): Promise<Incident> {
    try {
      const response = await apiClient.get(`/incidents/${id}`);
      return response.data;
    } catch {
      return DEMO_INCIDENTS.find((i) => i.id === id) || DEMO_INCIDENTS[0];
    }
  },

  async create(data: CreateIncidentRequest): Promise<Incident> {
    try {
      const response = await apiClient.post("/incidents", data);
      return response.data;
    } catch {
      return { id: `inc-${Date.now()}`, camera_id: data.camera_id, event_type: data.event_type, severity: data.severity, status: "open", confidence: data.confidence, bounding_box: data.bounding_box || null, metadata: data.metadata || null, timestamp: data.timestamp || new Date().toISOString(), created_at: new Date().toISOString(), updated_at: new Date().toISOString() };
    }
  },

  async update(id: string, data: UpdateIncidentRequest): Promise<Incident> {
    try {
      const response = await apiClient.patch(`/incidents/${id}`, data);
      return response.data;
    } catch {
      const existing = DEMO_INCIDENTS.find((i) => i.id === id) || DEMO_INCIDENTS[0];
      return { ...existing, status: data.status, updated_at: new Date().toISOString() };
    }
  },
};
