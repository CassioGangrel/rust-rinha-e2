use clap::Parser;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use time::OffsetDateTime;

#[derive(Parser, Debug)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
}

pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug, Deserialize)]
pub struct NewTransactionData {
    #[serde(rename = "valor")]
    pub value: i32,
    #[serde(rename = "tipo")]
    pub kind: String,
    #[serde(rename = "descricao")]
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct NewTransactionResultData {
    #[serde(rename = "limite")]
    pub limit: i32,
    #[serde(rename = "saldo")]
    pub value: i32,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct StatementBalance {
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "limite")]
    pub limit: i32,
    #[serde(rename = "data_extrato", with = "time::serde::rfc3339")]
    pub statement_date: OffsetDateTime,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct StatementTransaction {
    #[serde(rename = "valor")]
    pub value: i32,
    #[serde(rename = "tipo")]
    pub kind: i32,
    #[serde(rename = "descricao")]
    pub description: i64,
    #[serde(rename = "realizada_em", with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustumerStatement {
  pub balance: StatementBalance,
  pub last_transactions: Vec<StatementTransaction>,
}
