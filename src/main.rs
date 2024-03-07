use sqlx::{postgres::PgPoolOptions, FromRow, PgPool };
use clap::Parser;

#[derive(Parser, Debug)]
struct Config {
    #[clap(long, env)]
    db_url: String,
}

struct AppState {
    pool: PgPool
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

}
