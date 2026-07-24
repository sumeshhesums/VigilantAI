export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  per_page: number;
}

export interface ApiResponse<T> {
  data: T;
}

export interface ApiError {
  error: {
    code: string;
    message: string;
    details?: Array<{
      field: string;
      code: string;
      message: string;
    }>;
    request_id?: string;
    timestamp?: string;
  };
}

export interface PaginationParams {
  page?: number;
  per_page?: number;
}
