"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import { useRouter, usePathname } from "next/navigation";
import type { AuthUser, AuthState } from "@/types";
import { authService } from "@/services";

interface AuthContextType extends AuthState {
  login: (email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

const PUBLIC_ROUTES = ["/login"];

function AuthSpinner() {
  return (
    <div className="flex h-screen w-screen items-center justify-center bg-background">
      <div className="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent" />
    </div>
  );
}

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [state, setState] = useState<AuthState>({
    user: null,
    token: null,
    refreshToken: null,
    isAuthenticated: false,
    isLoading: true,
  });
  const router = useRouter();
  const pathname = usePathname();
  const mountedRef = useRef(false);

  useEffect(() => {
    const token = localStorage.getItem("access_token");
    const refreshToken = localStorage.getItem("refresh_token");
    const userStr = localStorage.getItem("user");

    if (token && refreshToken && userStr) {
      try {
        const user = JSON.parse(userStr) as AuthUser;
        setState({
          user,
          token,
          refreshToken,
          isAuthenticated: true,
          isLoading: false,
        });
      } catch {
        localStorage.clear();
        setState((s) => ({ ...s, isLoading: false }));
      }
    } else {
      setState((s) => ({ ...s, isLoading: false }));
    }
    mountedRef.current = true;
  }, []);

  useEffect(() => {
    if (state.isLoading) return;

    const isPublic = PUBLIC_ROUTES.includes(pathname);

    if (!state.isAuthenticated && !isPublic) {
      router.replace("/login");
    } else if (state.isAuthenticated && pathname === "/login") {
      router.replace("/");
    }
  }, [state.isAuthenticated, state.isLoading, pathname, router]);

  const login = useCallback(async (email: string, password: string) => {
    const authResponse = await authService.login({ email, password });

    localStorage.setItem("access_token", authResponse.access_token);
    localStorage.setItem("refresh_token", authResponse.refresh_token);

    const user = await authService.getMe();
    localStorage.setItem("user", JSON.stringify(user));

    setState({
      user,
      token: authResponse.access_token,
      refreshToken: authResponse.refresh_token,
      isAuthenticated: true,
      isLoading: false,
    });
  }, []);

  const logout = useCallback(async () => {
    try {
      await authService.logout();
    } catch {
      // Ignore logout errors
    } finally {
      localStorage.removeItem("access_token");
      localStorage.removeItem("refresh_token");
      localStorage.removeItem("user");
      setState({
        user: null,
        token: null,
        refreshToken: null,
        isAuthenticated: false,
        isLoading: false,
      });
      router.push("/login");
    }
  }, [router]);

  if (state.isLoading || !mountedRef.current) {
    return <AuthSpinner />;
  }

  const isPublic = PUBLIC_ROUTES.includes(pathname);

  if (!state.isAuthenticated && !isPublic) {
    return <AuthSpinner />;
  }

  if (state.isAuthenticated && pathname === "/login") {
    return <AuthSpinner />;
  }

  return (
    <AuthContext.Provider value={{ ...state, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}
