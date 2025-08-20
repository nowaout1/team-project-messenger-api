use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;

use crate::AppState;
use crate::auth::error::AuthError;
use crate::authentication_proto;

mod body;

pub async fn create_auth_router() -> anyhow::Result<axum::Router<AppState>> {
    let router = axum::Router::new()
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/validate", post(validate))
        .route("/api/v1/auth/refresh", post(refresh))
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/logout-all", post(logout_all));

    Ok(router)
}

async fn login(
    State(app): State<AppState>,
    Json(credentials): Json<body::LoginCredentials>,
) -> Result<impl IntoResponse, AuthError> {
    let mut service = app.authentication_client.lock().await;

    let credentials = authentication_proto::LoginRequest {
        username: credentials.username,
        password: credentials.password,
    };

    let session = service.login(credentials).await?;

    Ok((
        StatusCode::OK,
        Json(body::Session {
            access_token: session.access_token,
            refresh_token: session.refresh_token,
        }),
    ))
}

async fn register(
    State(app): State<AppState>,
    Json(credentials): Json<body::RegisterCredentials>,
) -> Result<impl IntoResponse, AuthError> {
    let mut service = app.authentication_client.lock().await;

    let credentials = authentication_proto::RegisterRequest {
        username: credentials.username,
        password: credentials.password,
    };

    let session = service.register(credentials).await?;

    Ok((
        StatusCode::CREATED,
        Json(body::Session {
            access_token: session.access_token,
            refresh_token: session.refresh_token,
        }),
    ))
}

async fn validate(
    State(app): State<AppState>,
    Json(token): Json<body::AccessToken>,
) -> Result<impl IntoResponse, AuthError> {
    let mut service = app.authentication_client.lock().await;

    let token_set = authentication_proto::ValidateRequest {
        access_token: token.access_token,
    };

    let res = service.validate(token_set).await?;

    Ok((
        StatusCode::OK,
        Json(body::IsValid {
            is_valid: res.is_valid,
        }),
    ))
}

async fn refresh(
    State(app): State<AppState>,
    Json(session): Json<body::Session>,
) -> Result<impl IntoResponse, AuthError> {
    let mut service = app.authentication_client.lock().await;

    let token_set = authentication_proto::RefreshRequest {
        access_token: session.access_token,
        refresh_token: session.refresh_token,
    };

    let res = service.refresh(token_set).await?;

    Ok((
        StatusCode::OK,
        Json(body::Session {
            access_token: res.access_token,
            refresh_token: res.refresh_token,
        }),
    ))
}

async fn logout(
    State(app): State<AppState>,
    Json(session): Json<body::Session>,
) -> Result<impl IntoResponse, AuthError> {
    let mut service = app.authentication_client.lock().await;

    let token_set = authentication_proto::LogoutRequest {
        access_token: session.access_token,
        refresh_token: session.refresh_token,
    };

    service.logout(token_set).await?;

    Ok(StatusCode::OK)
}

async fn logout_all(
    State(app): State<AppState>,
    Json(token): Json<body::AccessToken>,
) -> Result<impl IntoResponse, AuthError> {
    let mut service = app.authentication_client.lock().await;

    let token = authentication_proto::LogoutAllRequest {
        access_token: token.access_token,
    };

    service.logout_all(token).await?;

    Ok(StatusCode::OK)
}
