import apiClient from "@/lib/axios-client";
import type {
  Evidence,
  EvidenceFilters,
  PaginatedResponse,
} from "@/types";

export const evidenceService = {
  async listByIncident(
    incidentId: string,
    filters?: EvidenceFilters
  ): Promise<PaginatedResponse<Evidence>> {
    const response = await apiClient.get(`/incidents/${incidentId}/evidence`, {
      params: filters,
    });
    const raw = response.data;
    return { data: raw.evidence || raw.data || [], total: raw.total, page: raw.page, per_page: raw.per_page };
  },

  async getById(id: string): Promise<Evidence> {
    const response = await apiClient.get(`/evidence/${id}`);
    return response.data.data;
  },

  async upload(
    incidentId: string,
    file: File,
    onProgress?: (progress: number) => void
  ): Promise<Evidence> {
    const formData = new FormData();
    formData.append("file", file);

    const response = await apiClient.post(
      `/incidents/${incidentId}/evidence`,
      formData,
      {
        headers: { "Content-Type": "multipart/form-data" },
        onUploadProgress: (progressEvent) => {
          if (progressEvent.total && onProgress) {
            onProgress(
              Math.round((progressEvent.loaded * 100) / progressEvent.total)
            );
          }
        },
      }
    );
    return response.data.data;
  },

  async delete(id: string): Promise<void> {
    await apiClient.delete(`/evidence/${id}`);
  },
};
