use std::fmt;

#[derive(Debug, Clone)]
pub enum AIClientError {
    Offline { url: String },
    Timeout { url: String, timeout_ms: u64 },
    HttpServerError { status: u16, body: String },
    InvalidResponse { detail: String },
    MultipartBuild { detail: String },
    ImageOversized { size: usize, max: usize },
}

impl fmt::Display for AIClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AIClientError::Offline { url } => write!(f, "AI service offline: {url}"),
            AIClientError::Timeout { url, timeout_ms } => {
                write!(f, "AI service timeout after {timeout_ms}ms: {url}")
            }
            AIClientError::HttpServerError { status, body } => {
                write!(f, "AI service HTTP {status}: {body}")
            }
            AIClientError::InvalidResponse { detail } => {
                write!(f, "invalid AI response: {detail}")
            }
            AIClientError::MultipartBuild { detail } => {
                write!(f, "multipart build error: {detail}")
            }
            AIClientError::ImageOversized { size, max } => {
                write!(f, "image too large: {size} bytes (max {max})")
            }
        }
    }
}

impl std::error::Error for AIClientError {}

impl From<reqwest::Error> for AIClientError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            AIClientError::Timeout {
                url: e.url().map(|u| u.to_string()).unwrap_or_default(),
                timeout_ms: 0,
            }
        } else if e.is_connect() {
            AIClientError::Offline {
                url: e.url().map(|u| u.to_string()).unwrap_or_default(),
            }
        } else {
            AIClientError::InvalidResponse {
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
        let err = AIClientError::Offline {
            url: "http://localhost:8081".to_string(),
        };
        assert!(err.to_string().contains("offline"));
    }

    #[test]
    fn test_display_timeout() {
        let err = AIClientError::Timeout {
            url: "http://localhost:8081".to_string(),
            timeout_ms: 5000,
        };
        assert!(err.to_string().contains("5000ms"));
    }

    #[test]
    fn test_display_http_error() {
        let err = AIClientError::HttpServerError {
            status: 500,
            body: "internal error".to_string(),
        };
        assert!(err.to_string().contains("500"));
        assert!(err.to_string().contains("internal error"));
    }

    #[test]
    fn test_display_invalid_response() {
        let err = AIClientError::InvalidResponse {
            detail: "bad json".to_string(),
        };
        assert!(err.to_string().contains("bad json"));
    }

    #[test]
    fn test_display_multipart() {
        let err = AIClientError::MultipartBuild {
            detail: "missing file".to_string(),
        };
        assert!(err.to_string().contains("multipart"));
    }

    #[test]
    fn test_display_oversized() {
        let err = AIClientError::ImageOversized {
            size: 20_000_000,
            max: 10_000_000,
        };
        assert!(err.to_string().contains("20000000"));
    }

    #[test]
    fn test_is_error_trait() {
        let err = AIClientError::Offline {
            url: "x".to_string(),
        };
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_from_reqwest_timeout() {
        let mut builder = reqwest::Client::builder();
        builder = builder.timeout(std::time::Duration::from_millis(1));
        let client = builder.build().unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let err = rt.block_on(async {
            client
                .get("http://192.0.2.1:1/noscope")
                .send()
                .await
                .unwrap_err()
        });
        let ai_err: AIClientError = err.into();
        assert!(matches!(ai_err, AIClientError::Timeout { .. }));
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
        let ai_err: AIClientError = err.into();
        assert!(matches!(ai_err, AIClientError::Offline { .. }));
    }
}
