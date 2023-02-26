use std::{collections::HashMap, error::Error, fmt::Display, net::SocketAddr, path::PathBuf};

use axum::{
    body::{Body, Bytes},
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use axum_extra::routing::SpaRouter;
use clap::Parser;
use hyper::Request;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, query, PgPool};
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::{auth::AuthorizedUser, models::Player};

mod models;

mod auth;

mod all_players;
mod game_data;
mod login;
mod new_game;
mod new_player;
mod round_completed;

// use crate::auth::AuthorizedUser;

#[derive(Debug, clap::Parser)]
struct Args {
    /// Path to the dotenv file containing the required environment variables.
    #[clap(long)]
    dotenv_file_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt().init();
    dotenv::from_path(args.dotenv_file_path).unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&dotenv::var("DATABASE_URL")?)
        .await?;

    // `SpaRouter` is the easiest way to serve assets at a nested route like `/assets`
    // let app = Router::new()
    //     .route("/foo", get(|| async { "Hi from /foo" }))
    //     .merge(axum_extra::routing::SpaRouter::new("/assets", "."))
    //     .layer(TraceLayer::new_for_http());

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        // allow requests from any origin
        // .allow_origin(Any);
        // .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap());
        .allow_origin(AllowOrigin::mirror_request());

    let app = Router::new()
        .route("/new_game", get(new_game::new_game))
        .route("/login", get(login::get))
        .route("/all_players", get(all_players::get))
        .route("/game_data/:id", get(game_data::get))
        .route("/round_completed", post(round_completed::post))
        .route("/add_tsv", post(add_tsv))
        .route("/new_player/:name", post(new_player::post))
        // .merge(SpaRouter::new(
        //     "/rummy",
        //     "/home/benluelo/personal-projects/rummy_scores/frontend/public",
        // ))
        .layer(cors)
        .layer(Extension(pool));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    dbg!(addr);
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[tracing::instrument(skip(db, tsv_body))]
async fn add_tsv(
    auth: AuthorizedUser,
    Extension(db): Extension<PgPool>,
    tsv_body: Request<Body>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    dbg!();
    let body_reader = match hyper::body::to_bytes(tsv_body.into_body()).await {
        Ok(bytes) => bytes,
        Err(why) => return Err(why.to_string().into_response()),
    };
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter('\t' as u8)
        .from_reader(<Bytes as core::convert::AsRef<[u8]>>::as_ref(&body_reader));

    let mut players = match rdr.headers() {
        Ok(ok) => ok
            .into_iter()
            .map(|name| (name.to_string(), models::Score::default()))
            .collect::<HashMap<_, _>>(),
        Err(why) => return Err(why.to_string().into_response()),
    };

    for (idx, result) in rdr.records().enumerate() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let current_round_scores = result
            .as_ref()
            .map_err(ToString::to_string)
            .map_err(IntoResponse::into_response)?;

        for ((player, player_scores), score) in
            players.iter_mut().zip(current_round_scores.into_iter())
        {
            dbg!(player, score);
            let numeric_score =
                str::parse::<i32>(score).map_err(|why| why.to_string().into_response())?;

            if let Some(value) = player_scores[idx].replace(numeric_score) {
                return Err(
                    format!("attempted to set the same round ({idx}) twice").into_response()
                );
            }
        }
    }

    println!("{players:#?}");

    let existing_players_name_to_id_map = all_players::get(auth.clone(), Extension(db.clone()))
        .await
        .map_err(IntoResponse::into_response)?
        .0
        .into_iter()
        .map(|Player { id, name, .. }| (name, id))
        .collect::<HashMap<_, _>>();

    let mut tx = db
        .begin()
        .await
        .map_err(|why| why.to_string().into_response())?;

    let new_game_id = new_game::do_new_game(
        players
            .iter()
            .map(|(player, score)| {
                existing_players_name_to_id_map
                    .get(player)
                    .copied()
                    .ok_or(format!("player {player} does not exist").into_response())
            })
            .collect::<Result<_, _>>()?,
        &mut tx,
    )
    .await
    .map_err(IntoResponse::into_response)?;

    tracing::info!("created new game with id {}", new_game_id);

    for (player_name, score) in players {
        match query!(
            r#"
            UPDATE
                player_scores_to_player_and_game_id
            SET
                ace = $1,
                two = $2,
                three = $3,
                four = $4,
                five = $5,
                six = $6,
                seven = $7,
                eight = $8,
                nine = $9,
                ten = $10,
                jack = $11,
                queen = $12,
                king = $13
            WHERE
                player_id = $14
                AND
                game_id = $15
            "#,
            score.ace,
            score.two,
            score.three,
            score.four,
            score.five,
            score.six,
            score.seven,
            score.eight,
            score.nine,
            score.ten,
            score.jack,
            score.queen,
            score.king,
            existing_players_name_to_id_map[&player_name],
            new_game_id,
        )
        .execute(&mut tx)
        .await
        {
            Ok(_) => {}
            Err(why) => return Err(why.to_string().into_response()),
        }
    }

    match tx.commit().await {
        Ok(_) => {}
        Err(why) => return Err(why.to_string().into_response()),
    };

    Ok("success".into_response())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundCompleted {
    game_id: i32,
    round: Round,
    // player id -> score
    scores: HashMap<i32, i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Round {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Display for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Round::Ace => "ace",
            Round::Two => "two",
            Round::Three => "three",
            Round::Four => "four",
            Round::Five => "five",
            Round::Six => "six",
            Round::Seven => "seven",
            Round::Eight => "eight",
            Round::Nine => "nine",
            Round::Ten => "ten",
            Round::Jack => "jack",
            Round::Queen => "queen",
            Round::King => "king",
        }
        .fmt(f)
    }
}
