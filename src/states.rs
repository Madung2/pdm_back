use sqlx::mysql::MySqlPool;
#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}