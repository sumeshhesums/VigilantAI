"use client";

import { useQuery } from "@tanstack/react-query";
import { healthService } from "@/services";

export function useHealth() {
  return useQuery({
    queryKey: ["health"],
    queryFn: () => healthService.getHealth(),
    refetchInterval: 30000,
  });
}
