use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use sqlx::{query_as, PgPool};

use crate::{
    auth::AuthorizedUser,
    models::{Game, ScoreWithPlayer},
};

#[tracing::instrument(skip_all)]
pub(crate) async fn get(
    _: AuthorizedUser,
    Path(id): Path<i32>,
    Extension(db): Extension<PgPool>,
) -> impl IntoResponse {
    match query_as!(
        ScoreWithPlayer,
        "
        SELECT
            player_scores_to_player_and_game_id.ace,
            player_scores_to_player_and_game_id.two,
            player_scores_to_player_and_game_id.three,
            player_scores_to_player_and_game_id.four,
            player_scores_to_player_and_game_id.five,
            player_scores_to_player_and_game_id.six,
            player_scores_to_player_and_game_id.seven,
            player_scores_to_player_and_game_id.eight,
            player_scores_to_player_and_game_id.nine,
            player_scores_to_player_and_game_id.ten,
            player_scores_to_player_and_game_id.jack,
            player_scores_to_player_and_game_id.queen,
            player_scores_to_player_and_game_id.king,
            players.name as player_name,
            players.id as player_id
        FROM
            player_scores_to_player_and_game_id
        INNER JOIN
            players
        ON
            players.id = player_scores_to_player_and_game_id.player_id
        WHERE
            game_id = $1
        ",
        id
    )
    .fetch_all(&db)
    .await
    {
        Ok(players_and_scores) => Json(Game {
            id,
            scores: players_and_scores, // .into_iter()
                                        // .map(|player_score| (player_score.player(), player_score.score()))
                                        // .collect(),
        })
        .into_response(),
        Err(why) => {
            tracing::error!("error selecting from the database: {}", why);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
