export interface HealthStatus {
  status: string;
  version: string;
  uptime_seconds: number;
  services: {
    database: ServiceHealth;
    cache: ServiceHealth;
    ai_engine: ServiceHealth;
    camera_gateway: ServiceHealth;
    evidence_store: ServiceHealth;
  };
}

export interface ServiceHealth {
  status: string;
  latency_ms?: number;
  active_streams?: number;
  used_gb?: number;
}
