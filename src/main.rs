use axum::{ Router, routing::get, extract::State, };
use sqlx::{postgres::PgPoolOptions, PgPool };
use clap::Parser;
use std::{os::linux::raw::stat, sync::Arc};

#[derive(Parser, Debug)]
struct Config {
    #[clap(long, env)]
    db_url: String,
}

struct AppState {
    pub pool: PgPool
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config: Config = Config::parse();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.db_url)
        .await
        .expect("Não foi possivel conectar ao banco de dados!");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Não foi possivel executar as migrations");

    let state = AppState { pool };

    let router = Router::new()
        .route("/", get(|| async {"Hello, world!"}))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();

}
