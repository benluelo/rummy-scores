use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use sqlx::{query_as, PgPool};

use crate::auth::AuthorizedUser;
use crate::models::Player;

#[tracing::instrument(skip_all)]
pub(crate) async fn post(
    _: AuthorizedUser,
    Path(name): Path<String>,
    Extension(db): Extension<PgPool>,
) -> impl IntoResponse {
    match query_as!(
        Player,
        "
        INSERT INTO
            players(name)
        VALUES
            ($1)
        RETURNING
            *
        ",
        name.to_lowercase()
    )
    .fetch_one(&db)
    .await
    {
        Ok(player) => Json(player).into_response(),
        Err(why) => {
            tracing::error!("error selecting from the database: {}", why);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
