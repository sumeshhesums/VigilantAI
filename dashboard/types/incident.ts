export type IncidentSeverity = "low" | "medium" | "high" | "critical";
export type IncidentStatus = "open" | "acknowledged" | "resolved" | "false_positive";

export interface Incident {
  id: string;
  camera_id: string;
  timestamp: string;
  severity: IncidentSeverity;
  status: IncidentStatus;
  event_type: string;
  confidence: number;
  bounding_box: BoundingBox | null;
  metadata: Record<string, unknown> | null;
  created_at: string;
  updated_at: string;
}

export interface BoundingBox {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface CreateIncidentRequest {
  camera_id: string;
  timestamp?: string;
  severity: IncidentSeverity;
  event_type: string;
  confidence: number;
  bounding_box?: BoundingBox;
  metadata?: Record<string, unknown>;
}

export interface UpdateIncidentRequest {
  status: IncidentStatus;
}

export interface IncidentFilters {
  camera_id?: string;
  severity?: IncidentSeverity;
  status?: IncidentStatus;
  event_type?: string;
  since?: string;
  until?: string;
  page?: number;
  per_page?: number;
}
