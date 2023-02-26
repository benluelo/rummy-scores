use axum::http::StatusCode;
use axum::Json;
use sqlx::{query_as, PgPool};
use axum::Extension;

use crate::auth::AuthorizedUser;
use super::Player;

#[tracing::instrument(skip_all)]
pub(crate) async fn get(
    _: AuthorizedUser,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Player>>, StatusCode> {
    match query_as!(
        Player,
        r#"
        SELECT * FROM players
        "#,
    )
    .fetch_all(&db)
    .await
    {
        Ok(all_players) => Ok(Json(all_players)),
        Err(why) => {
            tracing::error!("error selecting from the database: {}", why);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
