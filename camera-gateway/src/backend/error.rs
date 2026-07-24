use std::fmt;

#[derive(Debug, Clone)]
pub enum BackendClientError {
    Offline { url: String },
    Timeout { url: String, timeout_ms: u64 },
    HttpError { status: u16, body: String },
    Unauthorized { detail: String },
    Conflict { detail: String },
    InvalidResponse { detail: String },
    Serialization { detail: String },
}

impl fmt::Display for BackendClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackendClientError::Offline { url } => write!(f, "backend offline: {url}"),
            BackendClientError::Timeout { url, timeout_ms } => {
                write!(f, "backend timeout after {timeout_ms}ms: {url}")
            }
            BackendClientError::HttpError { status, body } => {
                write!(f, "backend HTTP {status}: {body}")
            }
            BackendClientError::Unauthorized { detail } => {
                write!(f, "backend unauthorized: {detail}")
            }
            BackendClientError::Conflict { detail } => {
                write!(f, "backend conflict: {detail}")
            }
            BackendClientError::InvalidResponse { detail } => {
                write!(f, "invalid backend response: {detail}")
            }
            BackendClientError::Serialization { detail } => {
                write!(f, "backend serialization error: {detail}")
            }
        }
    }
}

impl std::error::Error for BackendClientError {}

impl From<reqwest::Error> for BackendClientError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            BackendClientError::Timeout {
                url: e.url().map(|u| u.to_string()).unwrap_or_default(),
                timeout_ms: 0,
            }
        } else if e.is_connect() {
            BackendClientError::Offline {
                url: e.url().map(|u| u.to_string()).unwrap_or_default(),
            }
        } else {
            BackendClientError::InvalidResponse {
                detail: e.to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_offline() {
        let err = BackendClientError::Offline {
            url: "http://localhost:8080".to_string(),
        };
        assert!(err.to_string().contains("offline"));
        assert!(err.to_string().contains("8080"));
    }

    #[test]
    fn test_display_timeout() {
        let err = BackendClientError::Timeout {
            url: "http://localhost:8080".to_string(),
            timeout_ms: 5000,
        };
        assert!(err.to_string().contains("5000ms"));
    }

    #[test]
    fn test_display_http_error() {
        let err = BackendClientError::HttpError {
            status: 500,
            body: "internal error".to_string(),
        };
        assert!(err.to_string().contains("500"));
        assert!(err.to_string().contains("internal error"));
    }

    #[test]
    fn test_display_unauthorized() {
        let err = BackendClientError::Unauthorized {
            detail: "missing token".to_string(),
        };
        assert!(err.to_string().contains("unauthorized"));
        assert!(err.to_string().contains("missing token"));
    }

    #[test]
    fn test_display_conflict() {
        let err = BackendClientError::Conflict {
            detail: "duplicate incident".to_string(),
        };
        assert!(err.to_string().contains("conflict"));
    }

    #[test]
    fn test_display_invalid_response() {
        let err = BackendClientError::InvalidResponse {
            detail: "bad json".to_string(),
        };
        assert!(err.to_string().contains("bad json"));
    }

    #[test]
    fn test_display_serialization() {
        let err = BackendClientError::Serialization {
            detail: "missing field".to_string(),
        };
        assert!(err.to_string().contains("serialization"));
    }

    #[test]
    fn test_is_error_trait() {
        let err = BackendClientError::Offline {
            url: "x".to_string(),
        };
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_from_reqwest_timeout() {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(1))
            .build()
            .unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let err = rt.block_on(async {
            client
                .get("http://192.0.2.1:1/noscope")
                .send()
                .await
                .unwrap_err()
        });
        let backend_err: BackendClientError = err.into();
        assert!(matches!(backend_err, BackendClientError::Timeout { .. }));
    }

    #[test]
    fn test_from_reqwest_connect() {
        let client = reqwest::Client::new();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let err = rt.block_on(async {
            client
                .get("http://127.0.0.1:19999/health")
                .send()
                .await
                .unwrap_err()
        });
        let backend_err: BackendClientError = err.into();
        assert!(matches!(backend_err, BackendClientError::Offline { .. }));
    }
}
