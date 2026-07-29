use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::dto::notification::{
    MarkAllReadResponse, NotificationListResponse, NotificationPaginationParams,
    NotificationResponse, SendNotificationRequest,
};
use crate::errors::AppError;
use crate::middleware::auth::{AuthUser, UserRoles};
use crate::models::{Notification, NotificationChannel};
use crate::rbac::guards::require_any_role;
use crate::rbac::roles::Role;
use crate::repository::notification_repository::PostgresNotificationRepository;
use crate::services::NotificationService;
use crate::state::AppState;

fn notification_response(notification: Notification) -> NotificationResponse {
    NotificationResponse {
        id: notification.id,
        incident_id: notification.incident_id,
        channel: NotificationChannel::from_db_str(&notification.channel)
            .unwrap_or(NotificationChannel::Webhook),
        recipient: notification.recipient,
        status: crate::models::NotificationStatus::from_db_str(&notification.status)
            .unwrap_or(crate::models::NotificationStatus::Pending),
        attempts: notification.attempts,
        response_code: notification.response_code,
        error_message: notification.error_message,
        created_at: notification.created_at,
        sent_at: notification.sent_at,
    }
}

fn make_service(state: &AppState) -> NotificationService<PostgresNotificationRepository> {
    let repo = PostgresNotificationRepository;
    NotificationService::new(repo, state.config.notification.clone())
}

// ---------------------------------------------------------------------------
// POST /api/v1/notifications/send
// Allowed: SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn send_notification(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Json(body): Json<SendNotificationRequest>,
) -> Result<(StatusCode, Json<NotificationResponse>), AppError> {
    require_any_role(&roles, &[Role::SecurityAdmin, Role::SystemAdmin])?;

    let service = make_service(&state);

    let create = crate::models::CreateNotification {
        incident_id: body.incident_id,
        channel: body.channel,
        recipient: body.recipient,
    };

    let notification = service
        .send_notification(&state.postgres_pool, &create)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("no notifier registered") {
                AppError::Conflict(msg)
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok((
        StatusCode::CREATED,
        Json(notification_response(notification)),
    ))
}

// ---------------------------------------------------------------------------
// GET /api/v1/notifications
// Allowed: Viewer, Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn list_notifications(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Query(params): Query<NotificationPaginationParams>,
) -> Result<Json<NotificationListResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let service = make_service(&state);

    let response = service
        .history(&state.postgres_pool, &params)
        .await
        .map_err(AppError::Internal)?;

    Ok(Json(response))
}

// ---------------------------------------------------------------------------
// GET /api/v1/notifications/:id
// Allowed: Viewer, Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn get_notification(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<NotificationResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let service = make_service(&state);

    let notification = service.get(&state.postgres_pool, id).await.map_err(|e| {
        if e.to_string().contains("not found") {
            AppError::NotFound
        } else {
            AppError::Internal(e)
        }
    })?;

    Ok(Json(notification_response(notification)))
}

// ---------------------------------------------------------------------------
// POST /api/v1/notifications/retry
// Allowed: Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn retry_notifications(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
) -> Result<Json<Vec<NotificationResponse>>, AppError> {
    require_any_role(
        &roles,
        &[Role::Operator, Role::SecurityAdmin, Role::SystemAdmin],
    )?;

    let service = make_service(&state);

    let retried = service
        .retry_failed(&state.postgres_pool)
        .await
        .map_err(AppError::Internal)?;

    let responses: Vec<NotificationResponse> =
        retried.into_iter().map(notification_response).collect();

    Ok(Json(responses))
}

// ---------------------------------------------------------------------------
// PUT /api/v1/notifications/:id/read
// Allowed: Viewer, Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn mark_notification_read(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<NotificationResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let service = make_service(&state);

    let notification = service
        .mark_as_read(&state.postgres_pool, id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                AppError::NotFound
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(Json(notification_response(notification)))
}

// ---------------------------------------------------------------------------
// PUT /api/v1/notifications/mark-all-read
// Allowed: Viewer, Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn mark_all_notifications_read(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
) -> Result<Json<MarkAllReadResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let service = make_service(&state);

    let affected = service
        .mark_all_as_read(&state.postgres_pool)
        .await
        .map_err(AppError::Internal)?;

    Ok(Json(MarkAllReadResponse { affected }))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
