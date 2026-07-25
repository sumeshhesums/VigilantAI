use prometheus::{
    Encoder, HistogramVec, IntCounter, IntCounterVec, IntGauge, Registry, TextEncoder,
};

#[derive(Clone)]
pub struct AppMetrics {
    pub registry: Registry,
    pub http_requests_total: IntCounterVec,
    pub http_request_duration_seconds: HistogramVec,
    pub jwt_auth_success_total: IntCounter,
    pub jwt_auth_failure_total: IntCounter,
    pub rbac_authorization_failures_total: IntCounter,
    pub incidents_created_total: IntCounter,
    pub evidence_uploads_total: IntCounter,
    pub notifications_sent_total: IntCounter,
    pub db_query_duration_seconds: HistogramVec,
    pub active_connections: IntGauge,
}

impl Default for AppMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl AppMetrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let http_requests_total = IntCounterVec::new(
            prometheus::opts!("vigilantai_http_requests_total", "Total HTTP requests"),
            &["method", "endpoint", "status"],
        )
        .unwrap();

        let http_request_duration_seconds = HistogramVec::new(
            prometheus::histogram_opts!(
                "vigilantai_http_request_duration_seconds",
                "HTTP request duration in seconds"
            )
            .buckets(vec![
                0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ]),
            &["method", "endpoint"],
        )
        .unwrap();

        let jwt_auth_success_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_jwt_auth_success_total",
            "JWT authentication successes"
        ))
        .unwrap();

        let jwt_auth_failure_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_jwt_auth_failure_total",
            "JWT authentication failures"
        ))
        .unwrap();

        let rbac_authorization_failures_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_rbac_authorization_failures_total",
            "RBAC authorization failures"
        ))
        .unwrap();

        let incidents_created_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_incidents_created_total",
            "Incidents created"
        ))
        .unwrap();

        let evidence_uploads_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_evidence_uploads_total",
            "Evidence uploads"
        ))
        .unwrap();

        let notifications_sent_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_notifications_sent_total",
            "Notifications sent"
        ))
        .unwrap();

        let db_query_duration_seconds = HistogramVec::new(
            prometheus::histogram_opts!(
                "vigilantai_db_query_duration_seconds",
                "Database query duration in seconds"
            )
            .buckets(vec![
                0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5,
            ]),
            &["query"],
        )
        .unwrap();

        let active_connections = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_active_connections",
            "Active connections"
        ))
        .unwrap();

        registry
            .register(Box::new(http_requests_total.clone()))
            .unwrap();
        registry
            .register(Box::new(http_request_duration_seconds.clone()))
            .unwrap();
        registry
            .register(Box::new(jwt_auth_success_total.clone()))
            .unwrap();
        registry
            .register(Box::new(jwt_auth_failure_total.clone()))
            .unwrap();
        registry
            .register(Box::new(rbac_authorization_failures_total.clone()))
            .unwrap();
        registry
            .register(Box::new(incidents_created_total.clone()))
            .unwrap();
        registry
            .register(Box::new(evidence_uploads_total.clone()))
            .unwrap();
        registry
            .register(Box::new(notifications_sent_total.clone()))
            .unwrap();
        registry
            .register(Box::new(db_query_duration_seconds.clone()))
            .unwrap();
        registry
            .register(Box::new(active_connections.clone()))
            .unwrap();

        Self {
            registry,
            http_requests_total,
            http_request_duration_seconds,
            jwt_auth_success_total,
            jwt_auth_failure_total,
            rbac_authorization_failures_total,
            incidents_created_total,
            evidence_uploads_total,
            notifications_sent_total,
            db_query_duration_seconds,
            active_connections,
        }
    }

    pub fn encode_metrics(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}
