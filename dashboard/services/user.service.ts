import apiClient from "@/lib/axios-client";
import type {
  User,
  CreateUserRequest,
  UpdateUserRequest,
  AssignRoleRequest,
  PaginatedResponse,
  PaginationParams,
} from "@/types";

export const userService = {
  async list(
    params?: PaginationParams
  ): Promise<PaginatedResponse<User>> {
    const response = await apiClient.get("/users", { params });
    const raw = response.data;
    return { data: raw.users || raw.data || [], total: raw.total, page: raw.page, per_page: raw.per_page };
  },

  async getById(id: string): Promise<User> {
    const response = await apiClient.get(`/users/${id}`);
    return response.data.data;
  },

  async create(data: CreateUserRequest): Promise<User> {
    const response = await apiClient.post("/users", data);
    return response.data.data;
  },

  async update(id: string, data: UpdateUserRequest): Promise<User> {
    const response = await apiClient.patch(`/users/${id}`, data);
    return response.data.data;
  },

  async delete(id: string): Promise<void> {
    await apiClient.delete(`/users/${id}`);
  },

  async assignRole(id: string, data: AssignRoleRequest): Promise<void> {
    await apiClient.post(`/users/${id}/roles`, data);
  },

  async removeRole(id: string, data: AssignRoleRequest): Promise<void> {
    await apiClient.delete(`/users/${id}/roles`, { data });
  },
};
