use std::time::Duration;

use bytes::Bytes;
use reqwest::Client;
use uuid::Uuid;

use crate::config::BackendConfig;

use super::error::BackendClientError;
use super::models::{
    EvidenceResponse, IncidentRequest, IncidentResponse, NotificationRequest, NotificationResponse,
};

#[derive(Debug, Clone)]
pub struct BackendClient {
    client: Client,
    base_url: String,
    timeout: Duration,
    auth_token: String,
}

impl BackendClient {
    pub fn new(config: BackendConfig) -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .build()
            .expect("failed to build reqwest client");

        Self {
            client,
            base_url: config.url,
            timeout: config.request_timeout,
            auth_token: config.auth_token,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn health(&self) -> Result<(), BackendClientError> {
        let url = format!("{}/health", self.base_url);
        let resp = self
            .client
            .get(&url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    BackendClientError::Offline {
                        url: self.base_url.clone(),
                    }
                } else if e.is_timeout() {
                    BackendClientError::Timeout {
                        url: self.base_url.clone(),
                        timeout_ms: self.timeout.as_millis() as u64,
                    }
                } else {
                    BackendClientError::from(e)
                }
            })?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            if status == 401 {
                Err(BackendClientError::Unauthorized { detail: body })
            } else {
                Err(BackendClientError::HttpError { status, body })
            }
        }
    }

    pub async fn publish_incident(
        &self,
        request: &IncidentRequest,
    ) -> Result<IncidentResponse, BackendClientError> {
        let url = format!("{}/api/v1/incidents", self.base_url);
        let resp = self
            .client
            .post(&url)
            .timeout(self.timeout)
            .bearer_auth(&self.auth_token)
            .json(request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    BackendClientError::Offline {
                        url: self.base_url.clone(),
                    }
                } else if e.is_timeout() {
                    BackendClientError::Timeout {
                        url: self.base_url.clone(),
                        timeout_ms: self.timeout.as_millis() as u64,
                    }
                } else {
                    BackendClientError::from(e)
                }
            })?;

        let status = resp.status();

        if status == reqwest::StatusCode::CREATED || status.is_success() {
            let body = resp.text().await.map_err(BackendClientError::from)?;
            serde_json::from_str::<IncidentResponse>(&body).map_err(|e| {
                BackendClientError::InvalidResponse {
                    detail: format!("failed to parse IncidentResponse: {e}"),
                }
            })
        } else if status == reqwest::StatusCode::UNAUTHORIZED {
            let body = resp.text().await.unwrap_or_default();
            Err(BackendClientError::Unauthorized { detail: body })
        } else if status == reqwest::StatusCode::CONFLICT {
            let body = resp.text().await.unwrap_or_default();
            Err(BackendClientError::Conflict { detail: body })
        } else {
            let body = resp.text().await.unwrap_or_default();
            Err(BackendClientError::HttpError {
                status: status.as_u16(),
                body,
            })
        }
    }

    /// Upload a JPEG frame as evidence for an incident.
    ///
    /// Sends a multipart POST to `POST /api/v1/incidents/{id}/evidence`. The
    /// backend stores the image and deduplicates by SHA-256 within the incident.
    pub async fn upload_evidence(
        &self,
        incident_id: Uuid,
        image: Bytes,
        file_name: &str,
        content_type: &str,
    ) -> Result<EvidenceResponse, BackendClientError> {
        let url = format!(
            "{}/api/v1/incidents/{incident_id}/evidence",
            self.base_url
        );

        let part = reqwest::multipart::Part::bytes(image.to_vec())
            .file_name(file_name.to_string())
            .mime_str(content_type)
            .map_err(|e| BackendClientError::Serialization {
                detail: e.to_string(),
            })?;

        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("content_type", content_type.to_string());

        let resp = self
            .client
            .post(&url)
            .timeout(self.timeout)
            .bearer_auth(&self.auth_token)
            .multipart(form)
            .send()
            .await
            .map_err(|e| self.map_transport_error(e))?;

        let status = resp.status();
        if status.is_success() {
            let body = resp.text().await.map_err(BackendClientError::from)?;
            serde_json::from_str::<EvidenceResponse>(&body).map_err(|e| {
                BackendClientError::InvalidResponse {
                    detail: format!("failed to parse EvidenceResponse: {e}"),
                }
            })
        } else {
            Err(self.map_status_error(resp).await)
        }
    }

    /// Send a notification for an incident.
    ///
    /// Sends `POST /api/v1/notifications/send`. An empty recipient makes the
    /// backend use the default webhook URL configured on its side.
    pub async fn send_notification(
        &self,
        incident_id: Uuid,
        recipient: &str,
    ) -> Result<NotificationResponse, BackendClientError> {
        let url = format!("{}/api/v1/notifications/send", self.base_url);

        let request = NotificationRequest {
            incident_id,
            channel: super::models::NotificationChannel::Webhook,
            recipient: recipient.to_string(),
        };

        let resp = self
            .client
            .post(&url)
            .timeout(self.timeout)
            .bearer_auth(&self.auth_token)
            .json(&request)
            .send()
            .await
            .map_err(|e| self.map_transport_error(e))?;

        let status = resp.status();
        if status.is_success() {
            let body = resp.text().await.map_err(BackendClientError::from)?;
            serde_json::from_str::<NotificationResponse>(&body).map_err(|e| {
                BackendClientError::InvalidResponse {
                    detail: format!("failed to parse NotificationResponse: {e}"),
                }
            })
        } else {
            Err(self.map_status_error(resp).await)
        }
    }

    /// Map a transport (reqwest) error into a `BackendClientError`.
    fn map_transport_error(&self, e: reqwest::Error) -> BackendClientError {
        if e.is_connect() {
            BackendClientError::Offline {
                url: self.base_url.clone(),
            }
        } else if e.is_timeout() {
            BackendClientError::Timeout {
                url: self.base_url.clone(),
                timeout_ms: self.timeout.as_millis() as u64,
            }
        } else {
            BackendClientError::from(e)
        }
    }

    /// Map a non-success HTTP status into a `BackendClientError`.
    async fn map_status_error(&self, resp: reqwest::Response) -> BackendClientError {
        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        match status {
            401 => BackendClientError::Unauthorized { detail: body },
            409 => BackendClientError::Conflict { detail: body },
            _ => BackendClientError::HttpError { status, body },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use uuid::Uuid;

    fn test_config(url: &str) -> BackendConfig {
        BackendConfig {
            url: url.to_string(),
            request_timeout: Duration::from_secs(5),
            auth_token: "test-token".to_string(),
            auto_publish: true,
            publish_retries: 3,
            publish_evidence: true,
            publish_notifications: true,
            severity_mapping: std::collections::HashMap::new(),
        }
    }

    fn sample_request() -> IncidentRequest {
        IncidentRequest {
            camera_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            timestamp: None,
            severity: super::super::models::IncidentSeverity::Medium,
            event_type: "person".to_string(),
            confidence: 0.95,
            bounding_box: Some(super::super::models::BoundingBox {
                x1: 10.0,
                y1: 20.0,
                x2: 100.0,
                y2: 200.0,
            }),
            metadata: None,
        }
    }

    fn sample_response_json() -> &'static str {
        r#"{
            "id": "550e8400-e29b-41d4-a716-446655440001",
            "camera_id": "550e8400-e29b-41d4-a716-446655440000",
            "timestamp": "2024-01-01T00:00:00Z",
            "severity": "medium",
            "status": "open",
            "event_type": "person",
            "confidence": 0.95,
            "bounding_box": {"x1": 10.0, "y1": 20.0, "x2": 100.0, "y2": 200.0},
            "metadata": null,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }"#
    }

    #[tokio::test]
    async fn test_health_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/health");
            then.status(200).body("ok");
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        client.health().await.unwrap();
        mock.assert();
    }

    #[tokio::test]
    async fn test_health_offline() {
        let client = BackendClient::new(test_config("http://127.0.0.1:19993"));
        let result = client.health().await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::Offline { .. }
        ));
    }

    #[tokio::test]
    async fn test_health_500() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/health");
            then.status(500).body("internal error");
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let result = client.health().await;
        match result.unwrap_err() {
            BackendClientError::HttpError { status, body } => {
                assert_eq!(status, 500);
                assert_eq!(body, "internal error");
            }
            other => panic!("expected HttpError, got {other:?}"),
        }
        mock.assert();
    }

    #[tokio::test]
    async fn test_health_unauthorized() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/health");
            then.status(401).body("unauthorized");
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let result = client.health().await;
        match result.unwrap_err() {
            BackendClientError::Unauthorized { detail } => {
                assert_eq!(detail, "unauthorized");
            }
            other => panic!("expected Unauthorized, got {other:?}"),
        }
        mock.assert();
    }

    #[tokio::test]
    async fn test_publish_incident_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/incidents")
                .header("Authorization", "Bearer test-token");
            then.status(201)
                .header("content-type", "application/json")
                .body(sample_response_json());
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let req = sample_request();
        let resp = client.publish_incident(&req).await.unwrap();
        assert_eq!(resp.event_type, "person");
        assert_eq!(resp.status, "open");
        mock.assert();
    }

    #[tokio::test]
    async fn test_publish_incident_unauthorized() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/v1/incidents");
            then.status(401)
                .body(r#"{"error": "unauthorized", "status": 401}"#);
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let req = sample_request();
        let result = client.publish_incident(&req).await;
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::Unauthorized { .. }
        ));
        mock.assert();
    }

    #[tokio::test]
    async fn test_publish_incident_conflict() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/v1/incidents");
            then.status(409)
                .body(r#"{"error": "duplicate incident", "status": 409}"#);
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let req = sample_request();
        let result = client.publish_incident(&req).await;
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::Conflict { .. }
        ));
        mock.assert();
    }

    #[tokio::test]
    async fn test_publish_incident_500() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/v1/incidents");
            then.status(500).body("database error");
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let req = sample_request();
        let result = client.publish_incident(&req).await;
        match result.unwrap_err() {
            BackendClientError::HttpError { status, body } => {
                assert_eq!(status, 500);
                assert_eq!(body, "database error");
            }
            other => panic!("expected HttpError, got {other:?}"),
        }
        mock.assert();
    }

    #[tokio::test]
    async fn test_publish_incident_timeout() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/v1/incidents");
            then.status(201)
                .delay(std::time::Duration::from_secs(60))
                .body(sample_response_json());
        });

        let config = BackendConfig {
            url: server.base_url(),
            request_timeout: Duration::from_millis(50),
            auth_token: "test-token".to_string(),
            auto_publish: true,
            publish_retries: 3,
            publish_evidence: true,
            publish_notifications: true,
            severity_mapping: std::collections::HashMap::new(),
        };
        let client = BackendClient::new(config);
        let req = sample_request();
        let result = client.publish_incident(&req).await;
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::Timeout { .. }
        ));
        mock.assert();
    }

    #[tokio::test]
    async fn test_publish_incident_offline() {
        let client = BackendClient::new(test_config("http://127.0.0.1:19992"));
        let req = sample_request();
        let result = client.publish_incident(&req).await;
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::Offline { .. }
        ));
    }

    #[tokio::test]
    async fn test_publish_incident_invalid_json_response() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/v1/incidents");
            then.status(201)
                .header("content-type", "application/json")
                .body("not json");
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let req = sample_request();
        let result = client.publish_incident(&req).await;
        match result.unwrap_err() {
            BackendClientError::InvalidResponse { detail } => {
                assert!(detail.contains("parse"));
            }
            other => panic!("expected InvalidResponse, got {other:?}"),
        }
        mock.assert();
    }

    #[tokio::test]
    async fn test_base_url() {
        let client = BackendClient::new(test_config("http://backend:8080"));
        assert_eq!(client.base_url(), "http://backend:8080");
    }

    fn sample_evidence_json() -> &'static str {
        r#"{
            "id": "550e8400-e29b-41d4-a716-446655440002",
            "incident_id": "550e8400-e29b-41d4-a716-446655440001",
            "file_name": "frame.jpg",
            "content_type": "image/jpeg",
            "file_size": 1024,
            "sha256": "abc123",
            "width": 640,
            "height": 480,
            "created_at": "2024-01-01T00:00:00Z"
        }"#
    }

    fn sample_notification_json() -> &'static str {
        r#"{
            "id": "550e8400-e29b-41d4-a716-446655440003",
            "incident_id": "550e8400-e29b-41d4-a716-446655440001",
            "channel": "webhook",
            "recipient": "",
            "status": "sent",
            "attempts": 1,
            "response_code": 200,
            "error_message": null,
            "created_at": "2024-01-01T00:00:00Z",
            "sent_at": "2024-01-01T00:00:00Z"
        }"#
    }

    #[tokio::test]
    async fn test_upload_evidence_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/incidents/550e8400-e29b-41d4-a716-446655440001/evidence")
                .header("Authorization", "Bearer test-token")
                .body_contains("frame.jpg");
            then.status(201)
                .header("content-type", "application/json")
                .body(sample_evidence_json());
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let incident_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap();
        let img = bytes::Bytes::from_static(&[0xFF, 0xD8, 0xFF, 0xD9]);
        let resp = client
            .upload_evidence(incident_id, img, "frame.jpg", "image/jpeg")
            .await
            .unwrap();
        assert_eq!(resp.sha256, "abc123");
        assert_eq!(resp.width, Some(640));
        mock.assert();
    }

    #[tokio::test]
    async fn test_upload_evidence_offline() {
        let client = BackendClient::new(test_config("http://127.0.0.1:19991"));
        let incident_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap();
        let img = bytes::Bytes::from_static(&[0xFF, 0xD8, 0xFF, 0xD9]);
        let result = client
            .upload_evidence(incident_id, img, "frame.jpg", "image/jpeg")
            .await;
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::Offline { .. }
        ));
    }

    #[tokio::test]
    async fn test_upload_evidence_unauthorized() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/incidents/550e8400-e29b-41d4-a716-446655440001/evidence");
            then.status(401).body(r#"{"error": "unauthorized", "status": 401}"#);
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let incident_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap();
        let img = bytes::Bytes::from_static(&[0xFF, 0xD8, 0xFF, 0xD9]);
        let result = client
            .upload_evidence(incident_id, img, "frame.jpg", "image/jpeg")
            .await;
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::Unauthorized { .. }
        ));
        mock.assert();
    }

    #[tokio::test]
    async fn test_upload_evidence_conflict() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/incidents/550e8400-e29b-41d4-a716-446655440001/evidence");
            then.status(409).body("unsupported content type");
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let incident_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap();
        let img = bytes::Bytes::from_static(&[0xFF, 0xD8, 0xFF, 0xD9]);
        let result = client
            .upload_evidence(incident_id, img, "frame.jpg", "image/jpeg")
            .await;
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::Conflict { .. }
        ));
        mock.assert();
    }

    #[tokio::test]
    async fn test_send_notification_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/notifications/send")
                .header("Authorization", "Bearer test-token")
                .json_body(serde_json::json!({
                    "incident_id": "550e8400-e29b-41d4-a716-446655440001",
                    "channel": "webhook",
                    "recipient": ""
                }));
            then.status(201)
                .header("content-type", "application/json")
                .body(sample_notification_json());
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let incident_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap();
        let resp = client.send_notification(incident_id, "").await.unwrap();
        assert_eq!(resp.status, "sent");
        assert_eq!(resp.channel, super::models::NotificationChannel::Webhook);
        mock.assert();
    }

    #[tokio::test]
    async fn test_send_notification_offline() {
        let client = BackendClient::new(test_config("http://127.0.0.1:19990"));
        let incident_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap();
        let result = client.send_notification(incident_id, "").await;
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::Offline { .. }
        ));
    }

    #[tokio::test]
    async fn test_send_notification_unauthorized() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/v1/notifications/send");
            then.status(401).body(r#"{"error": "unauthorized", "status": 401}"#);
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let incident_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap();
        let result = client.send_notification(incident_id, "").await;
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::Unauthorized { .. }
        ));
        mock.assert();
    }

    #[tokio::test]
    async fn test_send_notification_invalid_json_response() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/v1/notifications/send");
            then.status(201)
                .header("content-type", "application/json")
                .body("not json");
        });

        let client = BackendClient::new(test_config(&server.base_url()));
        let incident_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap();
        let result = client.send_notification(incident_id, "").await;
        assert!(matches!(
            result.unwrap_err(),
            BackendClientError::InvalidResponse { .. }
        ));
        mock.assert();
    }
}
