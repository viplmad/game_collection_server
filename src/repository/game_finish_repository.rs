use chrono::NaiveDate;
use sqlx::PgPool;

use crate::errors::RepositoryError;
use crate::query::game_finish_query;

use super::base::{execute, exists, fetch_all_single};

pub async fn find_all_by_game_id(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
) -> Result<Vec<NaiveDate>, RepositoryError> {
    let query = game_finish_query::select_all_by_user_id_and_game_id(user_id, game_id);
    fetch_all_single(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    date: NaiveDate,
) -> Result<(), RepositoryError> {
    let query = game_finish_query::insert(user_id, game_id, date);
    execute(pool, query).await
}

pub async fn delete(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    date: NaiveDate,
) -> Result<(), RepositoryError> {
    let query = game_finish_query::delete(user_id, game_id, date);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    date: NaiveDate,
) -> Result<bool, RepositoryError> {
    let query = game_finish_query::exists_by_id(user_id, game_id, date);
    exists::<_, NaiveDate>(pool, query).await
}