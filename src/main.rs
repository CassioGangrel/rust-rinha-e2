mod types;

use axum::{
    extract::{Json as ExtJson, Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};

use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use types::{
    AppState, CustumerStatement, NewTransactionData, NewTransactionResultData, StatementBalance,
    StatementTransaction,
};

use serde_json::json;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config: types::Config = types::Config::parse();

    let pool = PgPoolOptions::new()
        .max_connections(config.database_conn_pool_max)
        .min_connections(config.database_conn_pool_min)
        .connect(&config.database_url)
        .await
        .expect("Não foi possivel conectar ao banco de dados!");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Não foi possivel executar as migrations");

    let state = AppState { pool };

    let router = Router::new()
        .route("/clientes/:id/transacoes", post(new_transaction))
        .route("/clientes/:id/extrato", get(get_client_statement))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    println!("Inciando servidor na porta: {}", config.port);

    axum::serve(listener, router)
        .await
        .expect("Erro ao subir o servidor!");
}

const TRANSACTIONS_KIND: [&str; 2] = ["c", "d"];

async fn new_transaction(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    ExtJson(payload): ExtJson<NewTransactionData>,
) -> impl IntoResponse {
    if !check_id_in_range(id) {
        return Err(StatusCode::NOT_FOUND);
    }
    if !TRANSACTIONS_KIND.contains(&payload.kind.as_str()) {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    };
    if payload.description.len() < 1 || payload.description.len() > 10 {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    if payload.value < 0 {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    let mut tx = state.pool.begin().await.unwrap();
    let persit_transaction = sqlx::query(
        r#"
            INSERT INTO transactions 
            (customer_id, value, type, description)
            VALUES
            ($1, $2, $3, $4)
        "#,
    )
    .bind(id)
    .bind(payload.value)
    .bind(payload.kind)
    .bind(payload.description)
    .execute(&mut *tx)
    .await;

    match persit_transaction {
        Ok(_) => {
            tx.commit().await.expect("Error ao commitar a transação");
            let balance = sqlx::query_as::<_, NewTransactionResultData>(
                r#"
                    select b.value, b.credit as limit from balances b where b.customer_id = $1;
                "#,
            )
            .bind(id)
            .fetch_one(&state.pool)
            .await
            .expect("Não foi possivel buscar saldo");
            Ok(Json(json!(balance)))
        }
        Err(_) => {
            tx.rollback().await.unwrap();
            Err(StatusCode::UNPROCESSABLE_ENTITY)
        }
    }
}

async fn get_client_statement(
    Path(id): Path<i32>,
    State(store): State<AppState>,
) -> impl IntoResponse {
    if !check_id_in_range(id) {
        return Err(StatusCode::NOT_FOUND);
    }
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
    Ok(Json(json!(CustumerStatement {
        balance: data.0.expect("Erro ao carreger o saldo"),
        last_transactions: data.1.expect("Error ao carreger as transações"),
    })))
}

fn check_id_in_range(id: i32) -> bool {
    id > 0 && id <= 5
}
