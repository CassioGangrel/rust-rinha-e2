use clap::Parser;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use time::OffsetDateTime;

#[derive(Parser, Debug)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
}

#[derive(Clone)]
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

#[derive(Debug, Serialize, FromRow)]
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
    pub kind: String,
    #[serde(rename = "descricao")]
    pub description: String,
    #[serde(rename = "realizada_em", with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustumerStatement {
  #[serde(rename = "saldo")]
  pub balance: StatementBalance,
  #[serde(rename = "ultimas_transacoes")]
  pub last_transactions: Vec<StatementTransaction>,
}
