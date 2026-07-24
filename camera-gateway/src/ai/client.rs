use std::time::Duration;

use bytes::Bytes;
use reqwest::multipart;
use reqwest::Client;

use crate::config::AiConfig;

use super::error::AIClientError;
use super::models::DetectionResponse;

#[derive(Debug, Clone)]
pub struct AIClient {
    client: Client,
    base_url: String,
    timeout: Duration,
    max_frame_size: usize,
}

impl AIClient {
    pub fn new(config: AiConfig) -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .build()
            .expect("failed to build reqwest client");

        Self {
            client,
            base_url: config.service_url,
            timeout: config.request_timeout,
            max_frame_size: config.max_frame_size,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn health(&self) -> Result<(), AIClientError> {
        let url = format!("{}/health", self.base_url);
        let resp = self
            .client
            .get(&url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    AIClientError::Offline {
                        url: self.base_url.clone(),
                    }
                } else if e.is_timeout() {
                    AIClientError::Timeout {
                        url: self.base_url.clone(),
                        timeout_ms: self.timeout.as_millis() as u64,
                    }
                } else {
                    AIClientError::from(e)
                }
            })?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            Err(AIClientError::HttpServerError { status, body })
        }
    }

    pub async fn detect_frame(
        &self,
        image_bytes: Bytes,
        source: &str,
    ) -> Result<DetectionResponse, AIClientError> {
        if image_bytes.len() > self.max_frame_size {
            return Err(AIClientError::ImageOversized {
                size: image_bytes.len(),
                max: self.max_frame_size,
            });
        }

        let url = format!("{}/inference", self.base_url);
        let part = multipart::Part::bytes(image_bytes.to_vec())
            .file_name("frame.jpg")
            .mime_str("image/jpeg")
            .map_err(|e| AIClientError::MultipartBuild {
                detail: e.to_string(),
            })?;

        let form = multipart::Form::new()
            .part("file", part)
            .text("source", source.to_string());

        let resp = self
            .client
            .post(&url)
            .timeout(self.timeout)
            .multipart(form)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    AIClientError::Offline {
                        url: self.base_url.clone(),
                    }
                } else if e.is_timeout() {
                    AIClientError::Timeout {
                        url: self.base_url.clone(),
                        timeout_ms: self.timeout.as_millis() as u64,
                    }
                } else {
                    AIClientError::from(e)
                }
            })?;

        let status = resp.status();

        if status == reqwest::StatusCode::SERVICE_UNAVAILABLE {
            return Err(AIClientError::Offline {
                url: self.base_url.clone(),
            });
        }

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(AIClientError::HttpServerError {
                status: status.as_u16(),
                body,
            });
        }

        let body = resp.text().await.map_err(AIClientError::from)?;
        serde_json::from_str::<DetectionResponse>(&body).map_err(|e| {
            AIClientError::InvalidResponse {
                detail: format!("failed to parse DetectionResponse: {e}"),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    fn test_config(url: &str) -> AiConfig {
        AiConfig {
            service_url: url.to_string(),
            request_timeout: Duration::from_secs(5),
            jpeg_quality: 85,
            inference_interval: Duration::from_millis(500),
            max_frame_size: 10 * 1024 * 1024,
        }
    }

    fn sample_response_json() -> &'static str {
        r#"{
            "detections": [],
            "detection_count": 0,
            "image_size": {"width": 640, "height": 480},
            "processing_time_ms": 1.0,
            "inference_time_ms": 0.5,
            "metadata": {
                "model_name": "yolov8n",
                "image_size": {"width": 640, "height": 480},
                "source": "test",
                "confidence_threshold": 0.5,
                "iou_threshold": 0.45
            }
        }"#
    }

    #[tokio::test]
    async fn test_health_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/health");
            then.status(200).body("ok");
        });

        let client = AIClient::new(test_config(&server.base_url()));
        client.health().await.unwrap();
        mock.assert();
    }

    #[tokio::test]
    async fn test_health_offline() {
        let client = AIClient::new(test_config("http://127.0.0.1:19998"));
        let result = client.health().await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AIClientError::Offline { .. }));
    }

    #[tokio::test]
    async fn test_health_500() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/health");
            then.status(500).body("internal error");
        });

        let client = AIClient::new(test_config(&server.base_url()));
        let result = client.health().await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AIClientError::HttpServerError { status, body } => {
                assert_eq!(status, 500);
                assert_eq!(body, "internal error");
            }
            other => panic!("expected HttpServerError, got {other:?}"),
        }
        mock.assert();
    }

    #[tokio::test]
    async fn test_detect_frame_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/inference")
                .body_contains("frame.jpg");
            then.status(200)
                .header("content-type", "application/json")
                .body(sample_response_json());
        });

        let client = AIClient::new(test_config(&server.base_url()));
        let img = Bytes::from_static(&[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10]);
        let resp = client.detect_frame(img, "cam-1").await.unwrap();
        assert_eq!(resp.detection_count, 0);
        mock.assert();
    }

    #[tokio::test]
    async fn test_detect_frame_offline() {
        let client = AIClient::new(test_config("http://127.0.0.1:19997"));
        let img = Bytes::from_static(&[0xFF, 0xD8]);
        let result = client.detect_frame(img, "cam-1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detect_frame_500() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/inference");
            then.status(500).body("model not loaded");
        });

        let client = AIClient::new(test_config(&server.base_url()));
        let img = Bytes::from_static(&[0xFF, 0xD8]);
        let result = client.detect_frame(img, "cam-1").await;
        match result.unwrap_err() {
            AIClientError::HttpServerError { status, body } => {
                assert_eq!(status, 500);
                assert!(body.contains("model not loaded"));
            }
            other => panic!("expected HttpServerError, got {other:?}"),
        }
        mock.assert();
    }

    #[tokio::test]
    async fn test_detect_frame_invalid_json() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/inference");
            then.status(200)
                .header("content-type", "application/json")
                .body("not json");
        });

        let client = AIClient::new(test_config(&server.base_url()));
        let img = Bytes::from_static(&[0xFF, 0xD8]);
        let result = client.detect_frame(img, "cam-1").await;
        match result.unwrap_err() {
            AIClientError::InvalidResponse { detail } => {
                assert!(detail.contains("parse"));
            }
            other => panic!("expected InvalidResponse, got {other:?}"),
        }
        mock.assert();
    }

    #[tokio::test]
    async fn test_detect_frame_oversized() {
        let config = AiConfig {
            service_url: "http://unused".to_string(),
            request_timeout: Duration::from_secs(5),
            jpeg_quality: 85,
            inference_interval: Duration::from_millis(500),
            max_frame_size: 100,
        };
        let client = AIClient::new(config);
        let img = Bytes::from(vec![0u8; 101]);
        let result = client.detect_frame(img, "cam-1").await;
        match result.unwrap_err() {
            AIClientError::ImageOversized { size, max } => {
                assert_eq!(size, 101);
                assert_eq!(max, 100);
            }
            other => panic!("expected ImageOversized, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_detect_frame_503() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/inference");
            then.status(503).body("unavailable");
        });

        let client = AIClient::new(test_config(&server.base_url()));
        let img = Bytes::from_static(&[0xFF, 0xD8]);
        let result = client.detect_frame(img, "cam-1").await;
        assert!(matches!(result.unwrap_err(), AIClientError::Offline { .. }));
        mock.assert();
    }

    #[tokio::test]
    async fn test_base_url() {
        let client = AIClient::new(test_config("http://ai:8081"));
        assert_eq!(client.base_url(), "http://ai:8081");
    }

    #[tokio::test]
    async fn test_health_timeout() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/health");
            then.status(200)
                .delay(std::time::Duration::from_secs(60))
                .body("ok");
        });

        let config = AiConfig {
            service_url: server.base_url(),
            request_timeout: Duration::from_millis(50),
            jpeg_quality: 85,
            inference_interval: Duration::from_millis(500),
            max_frame_size: 10 * 1024 * 1024,
        };
        let client = AIClient::new(config);
        let result = client.health().await;
        assert!(result.is_err());
        mock.assert();
    }
}
