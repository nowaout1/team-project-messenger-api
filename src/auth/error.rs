use axum::http::StatusCode;

#[derive(
    thiserror::Error, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Copy,
)]
pub enum AuthError {
    #[error("already exists")]
    AlreadyExists,

    #[error("not found")]
    NotFound,

    #[error("invalid argument")]
    InvalidArgument,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("internal server error")]
    Internal,
}

impl From<tonic::Status> for AuthError {
    fn from(error: tonic::Status) -> Self {
        match error.code() {
            tonic::Code::AlreadyExists => Self::AlreadyExists,
            tonic::Code::NotFound => Self::NotFound,
            tonic::Code::InvalidArgument => Self::InvalidArgument,
            tonic::Code::Unauthenticated => Self::InvalidCredentials,
            _ => {
                tracing::error!("Auth error: {error:?}");
                Self::Internal
            }
        }
    }
}

impl axum::response::IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::AlreadyExists => StatusCode::CONFLICT,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InvalidArgument => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
