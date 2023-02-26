use axum::http::StatusCode;

use axum::response::IntoResponse;

use crate::auth::AuthorizedUser;

#[tracing::instrument]
pub(crate) async fn get(_: AuthorizedUser) -> impl IntoResponse {
    StatusCode::OK
}
