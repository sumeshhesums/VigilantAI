import apiClient from "@/lib/axios-client";
import { DEMO_ROLES } from "@/lib/demo-data";

export interface Role {
  id: string;
  name: string;
  description: string | null;
}

export const roleService = {
  async list(): Promise<Role[]> {
    try {
      const response = await apiClient.get("/roles");
      return response.data.roles || response.data.data || response.data;
    } catch {
      return DEMO_ROLES;
    }
  },

  async getById(id: string): Promise<Role> {
    try {
      const response = await apiClient.get(`/roles/${id}`);
      return response.data;
    } catch {
      return DEMO_ROLES.find((r) => r.id === id) || DEMO_ROLES[0];
    }
  },
};
