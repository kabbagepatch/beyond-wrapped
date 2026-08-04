use std::fs;

use tauri::{App, Manager};
use rusqlite::{Connection, Error::QueryReturnedNoRows, Params, Result, Row};

use crate::{AppError::{self, SqlError}, db::schema::SCHEMA};

pub fn init_db(app: &mut App) -> Connection {
  let dir = app.path().app_data_dir().unwrap();
  fs::create_dir_all(&dir).expect("Failed to create database");
  let db_path = dir.join("test.db");
  let conn = Connection::open(db_path).expect("Failed to open database");
  conn.execute_batch(SCHEMA).expect("Failed to initialize database");
  execute(&conn, "PRAGMA foreign_keys = ON;", []).expect("Failed to initialize database");

  conn
}

pub fn query_row<T, P, F>(conn: &Connection, sql: &str, params: P, f: F) -> Result<Option<T>, AppError> 
where P: Params, F: FnOnce(&Row<'_>) -> Result<T> {
  let mut statement = match conn.prepare(sql) {
      Ok(c) => c,
      Err(err) => {
          eprintln!("Failed to prepare statement: {err}");
          return Err(SqlError());
      }
  };

  match statement.query_row(params, f) {
    Ok(info) => Ok(Some(info)),
    Err(QueryReturnedNoRows) => Ok(None),
    Err(err) => {
      eprintln!("Failed to execute statement {sql}: {err}");
      return Err(SqlError());
    }
  }
}

pub fn query_map<T, P, F>(conn: &Connection, sql: &str, params: P, f: F) -> Result<Vec<T>, AppError> 
where P: Params, F: FnMut(&Row<'_>) -> Result<T> {
  let mut statement = match conn.prepare(sql) {
      Ok(c) => c,
      Err(err) => {
          eprintln!("Failed to prepare statement: {err}");
          return Err(SqlError());
      }
  };

  let mapped = statement.query_map(params, f).map_err(|err| {
      eprintln!("Failed to execute statement {sql}: {err}");
      SqlError()
  })?;

  let results = mapped.filter_map(|r| r.ok()).collect();
  Ok(results)
}

pub fn execute<P>(conn: &Connection, sql: &str, params: P) -> Result<usize, AppError> where P: Params {
  match conn.execute(sql, params) {
    Ok(info) => Ok(info),
    Err(err) => {
      eprintln!("Failed to execute statement {sql}: {err}");
      return Err(SqlError())
    }
  }
}
