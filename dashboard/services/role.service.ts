import apiClient from "@/lib/axios-client";

export interface Role {
  id: string;
  name: string;
  description: string | null;
}

export const roleService = {
  async list(): Promise<Role[]> {
    const response = await apiClient.get("/roles");
    return response.data.data;
  },

  async getById(id: string): Promise<Role> {
    const response = await apiClient.get(`/roles/${id}`);
    return response.data.data;
  },
};
