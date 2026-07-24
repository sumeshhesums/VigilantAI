export interface Evidence {
  id: string;
  incident_id: string;
  file_name: string;
  content_type: string;
  file_size: number;
  sha256: string;
  width: number | null;
  height: number | null;
  created_at: string;
}

export interface EvidenceFilters {
  page?: number;
  per_page?: number;
}
