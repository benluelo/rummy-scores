use axum::extract::Query;
use axum::Extension;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_with::{rust::StringWithSeparator, CommaSeparator};
use sqlx::{query, PgExecutor, PgPool};

use crate::auth::AuthorizedUser;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NewGameQueryParams {
    #[serde(with = "StringWithSeparator::<CommaSeparator>", alias = "players")]
    player_ids: Vec<i32>,
}

#[tracing::instrument(skip_all)]
pub(crate) async fn new_game(
    _: AuthorizedUser,
    Query(NewGameQueryParams { player_ids }): Query<NewGameQueryParams>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<i32>, StatusCode> {
    do_new_game(player_ids, &db).await.map(Json)
}

pub(crate) async fn do_new_game<'c, E: PgExecutor<'c>>(
    player_ids: Vec<i32>,
    db: E,
) -> Result<i32, StatusCode> {
    match query!(
        r#"
        SELECT create_new_game($1) as "id!: i32"
        "#,
        &player_ids
    )
    .fetch_one(db)
    .await
    {
        Ok(record) => Ok(record.id),
        Err(why) => {
            tracing::error!("error selecting from the database: {}", why);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
