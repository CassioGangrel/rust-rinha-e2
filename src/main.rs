mod types;

use axum::{
    extract::{Path, State,},
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};

use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use types::{
    AppState, CustumerStatement, StatementBalance, StatementTransaction,
};

use serde_json::json;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config: types::Config = types::Config::parse();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Não foi possivel conectar ao banco de dados!");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Não foi possivel executar as migrations");

    let state = AppState { pool };

    let router = Router::new()
        .route("/", get(root_handler))
        .route("/clientes/:id/transacoes", post(new_transaction))
        .route("/clientes/:id/extrato", get(get_client_statement))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn root_handler() -> impl IntoResponse {
    "To pronto para a briga, pode mandar vim!".to_string()
}

async fn new_transaction(

) -> impl IntoResponse {
    "criar nova transação".to_string()
}

async fn get_client_statement(
    Path(id): Path<i32>,
    State(store): State<Arc<AppState>>,
) -> impl IntoResponse {
    let last_10_transactions = sqlx::query_as::<_, StatementTransaction>(
        r#"
            select
                t.value,
                t.created_at,
                t.description,
                t.type as kind
            from
                transactions t
            where
                t.customer_id  = $1
            order by created_at desc
            limit 10;
        "#,
    )
    .bind(id)
    .fetch_all(&store.pool);

    let balance = sqlx::query_as::<_, StatementBalance>(
        r#"
            select 
                b.value as total,
                b.credit as limit,
                NOW() as statement_date
            from balances b
            where b.customer_id = $1;
        "#,
    )
    .bind(id)
    .fetch_one(&store.pool);

    let data = tokio::join!(balance, last_10_transactions);
    Json(json!(CustumerStatement {
        balance: data.0.expect("Erro ao carreger o saldo"),
        last_transactions: data.1.expect("Error ao carreger as transações"),
    }))
}
