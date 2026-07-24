export interface Camera {
  id: string;
  name: string;
  location: string | null;
  rtsp_url: string;
  status: string;
  enabled: boolean;
  fps: number | null;
  resolution: string | null;
  last_seen: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateCameraRequest {
  name: string;
  location?: string;
  rtsp_url: string;
  fps?: number;
  resolution?: string;
}

export interface UpdateCameraRequest {
  name?: string;
  location?: string | null;
  rtsp_url?: string;
  fps?: number | null;
  resolution?: string | null;
  enabled?: boolean;
}

export interface CameraFilters {
  status?: string;
  search?: string;
  page?: number;
  per_page?: number;
}
