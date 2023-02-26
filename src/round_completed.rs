use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use sqlx::query;
use sqlx::PgPool;

use super::RoundCompleted;

#[tracing::instrument(skip_all)]
pub(crate) async fn post(
    Json(RoundCompleted {
        game_id,
        round,
        scores,
    }): Json<RoundCompleted>,
    Extension(db): Extension<PgPool>,
) -> impl IntoResponse {
    let mut maybe_errors = None;
    for (player_id, score) in scores {
        // TODO: plpgsql function
        match query(&format!(
            r#"
            UPDATE player_scores_to_player_and_game_id SET {round} = {score} WHERE game_id = {game_id} AND player_id = {player_id}
            "#
        ))
        .execute(&db)
        .await
        {
            Ok(_) => {
                tracing::info!("successfully inserted score {} for player #{} in game {}", score, game_id, player_id);
            }
            Err(why) => {
                tracing::error!("error inserting score {} for player #{} in game {}: {}", score, game_id, player_id, &why);
                maybe_errors.get_or_insert_with(|| vec![]).push(why.to_string());
            }
        }
    }

    match maybe_errors {
        Some(errors) => (StatusCode::INTERNAL_SERVER_ERROR, Json(errors)).into_response(),
        None => StatusCode::OK.into_response(),
    }
}
