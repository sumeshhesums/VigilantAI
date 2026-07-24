"use client";

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { cameraService } from "@/services";
import type { CameraFilters, CreateCameraRequest, UpdateCameraRequest } from "@/types";

export function useCameras(filters?: CameraFilters) {
  return useQuery({
    queryKey: ["cameras", filters],
    queryFn: () => cameraService.list(filters),
  });
}

export function useCamera(id: string) {
  return useQuery({
    queryKey: ["camera", id],
    queryFn: () => cameraService.getById(id),
    enabled: !!id,
  });
}

export function useCreateCamera() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateCameraRequest) => cameraService.create(data),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["cameras"] }),
  });
}

export function useUpdateCamera() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateCameraRequest }) =>
      cameraService.update(id, data),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["cameras"] }),
  });
}

export function useDeleteCamera() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => cameraService.delete(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["cameras"] }),
  });
}

export function useEnableCamera() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => cameraService.enable(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["cameras"] }),
  });
}

export function useDisableCamera() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => cameraService.disable(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["cameras"] }),
  });
}
