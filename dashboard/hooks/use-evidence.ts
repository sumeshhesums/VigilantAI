"use client";

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { evidenceService } from "@/services";
import type { EvidenceFilters } from "@/types";

export function useEvidenceByIncident(incidentId: string, filters?: EvidenceFilters) {
  return useQuery({
    queryKey: ["evidence", incidentId, filters],
    queryFn: () => evidenceService.listByIncident(incidentId, filters),
    enabled: !!incidentId,
  });
}

export function useEvidence(id: string) {
  return useQuery({
    queryKey: ["evidence", id],
    queryFn: () => evidenceService.getById(id),
    enabled: !!id,
  });
}

export function useUploadEvidence() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      incidentId,
      file,
      onProgress,
    }: {
      incidentId: string;
      file: File;
      onProgress?: (progress: number) => void;
    }) => evidenceService.upload(incidentId, file, onProgress),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ["evidence", variables.incidentId] });
    },
  });
}

export function useDeleteEvidence() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => evidenceService.delete(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["evidence"] }),
  });
}
