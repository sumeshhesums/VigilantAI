import apiClient from "@/lib/axios-client";
import type {
  User,
  CreateUserRequest,
  UpdateUserRequest,
  AssignRoleRequest,
  PaginatedResponse,
  PaginationParams,
} from "@/types";
import { DEMO_USERS } from "@/lib/demo-data";

export const userService = {
  async list(
    params?: PaginationParams
  ): Promise<PaginatedResponse<User>> {
    try {
      const response = await apiClient.get("/users", { params });
      const raw = response.data;
      const data = raw.users || raw.data || [];
      if (data.length === 0) return { data: DEMO_USERS, total: DEMO_USERS.length, page: 1, per_page: 20 };
      return { data, total: raw.total ?? data.length, page: raw.page ?? 1, per_page: raw.per_page ?? 20 };
    } catch {
      return { data: DEMO_USERS, total: DEMO_USERS.length, page: 1, per_page: 20 };
    }
  },

  async getById(id: string): Promise<User> {
    try {
      const response = await apiClient.get(`/users/${id}`);
      return response.data;
    } catch {
      return DEMO_USERS.find((u) => u.id === id) || DEMO_USERS[0];
    }
  },

  async create(data: CreateUserRequest): Promise<User> {
    try {
      const response = await apiClient.post("/users", data);
      return response.data;
    } catch {
      return { id: `user-${Date.now()}`, email: data.email, first_name: data.first_name, last_name: data.last_name, is_active: true, roles: data.roles || ["viewer"], created_at: new Date().toISOString(), updated_at: new Date().toISOString() };
    }
  },

  async update(id: string, data: UpdateUserRequest): Promise<User> {
    try {
      const response = await apiClient.patch(`/users/${id}`, data);
      return response.data;
    } catch {
      const existing = DEMO_USERS.find((u) => u.id === id) || DEMO_USERS[0];
      return { ...existing, ...data, updated_at: new Date().toISOString() };
    }
  },

  async delete(id: string): Promise<void> {
    try { await apiClient.delete(`/users/${id}`); } catch { /* demo */ }
  },

  async assignRole(id: string, data: AssignRoleRequest): Promise<void> {
    try { await apiClient.post(`/users/${id}/roles`, data); } catch { /* demo */ }
  },

  async removeRole(id: string, data: AssignRoleRequest): Promise<void> {
    try { await apiClient.delete(`/users/${id}/roles`, { data }); } catch { /* demo */ }
  },
};
