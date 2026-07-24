import apiClient from "@/lib/axios-client";
import type {
  Incident,
  CreateIncidentRequest,
  UpdateIncidentRequest,
  IncidentFilters,
  PaginatedResponse,
} from "@/types";

export const incidentService = {
  async list(filters?: IncidentFilters): Promise<PaginatedResponse<Incident>> {
    const response = await apiClient.get("/incidents", { params: filters });
    const raw = response.data;
    return { data: raw.incidents || raw.data || [], total: raw.total, page: raw.page, per_page: raw.per_page };
  },

  async getById(id: string): Promise<Incident> {
    const response = await apiClient.get(`/incidents/${id}`);
    return response.data.data;
  },

  async create(data: CreateIncidentRequest): Promise<Incident> {
    const response = await apiClient.post("/incidents", data);
    return response.data.data;
  },

  async update(id: string, data: UpdateIncidentRequest): Promise<Incident> {
    const response = await apiClient.patch(`/incidents/${id}`, data);
    return response.data.data;
  },
};
