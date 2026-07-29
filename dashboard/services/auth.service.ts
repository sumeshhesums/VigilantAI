import apiClient from "@/lib/axios-client";
import type {
  LoginRequest,
  RegisterRequest,
  AuthResponse,
  AuthUser,
} from "@/types";

export const authService = {
  async login(data: LoginRequest): Promise<AuthResponse> {
    const response = await apiClient.post("/auth/login", data);
    return response.data;
  },

  async register(data: RegisterRequest): Promise<AuthUser> {
    const response = await apiClient.post("/auth/register", data);
    return response.data;
  },

  async refreshToken(refreshToken: string): Promise<AuthResponse> {
    const response = await apiClient.post("/auth/refresh", {
      refresh_token: refreshToken,
    });
    return response.data;
  },

  async logout(): Promise<void> {
    await apiClient.post("/auth/logout");
  },

  async getMe(): Promise<AuthUser> {
    const response = await apiClient.get("/auth/me");
    return response.data;
  },
};
