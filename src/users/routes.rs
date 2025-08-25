use axum::{Router, routing::{get, post}, Json};
use serde::{Serialize, Deserialize};
use crate::handlers::{http_handler::{ApiResponse, ApiError}};
use crate::utils::utils;
use tracing::error;
use std::sync::Arc;
use crate::states::AppState;


// model
#[derive(Serialize)]
pub struct UserModel {
    user_id : String,
    passwd: String
}

// model
#[derive(Serialize, Deserialize)]
pub struct LoginModel {
    user_id : String,
    passwd: String
}


async fn get_users() -> Json<Vec<UserModel>> {
    let users = vec![
        UserModel { user_id: "devLovers".to_string(), passwd: "hash1".to_string() },
        UserModel { user_id: "devLovers2".to_string(), passwd: "hash2".to_string() },
    ];
    Json(users)
}
async fn get_user() { /* ... */ }


fn split_db_hash(input: &str) ->(Option<&str>,&str) {
    if let Some(_idx) = input.find("$argon2") {
        if let Some((pre, hash)) = input.split_once('|') {
            return (Some(pre), hash);
        }
    }
    (None, input)
}


async fn post_login(Json(payload): Json<LoginModel>) -> Result<ApiResponse<LoginModel>, ApiError> {
    let pool = crate::settings::create_db_pool().await;
    let payload_user_id = &payload.user_id;
    let _payload_passwd  = &payload.passwd;

    let row = sqlx::query!("SELECT password FROM oc_users WHERE uid = ?", payload_user_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch user: {:?}", e);
            ApiError::Unauthorized
        })?;
    let db_hash_bytes = &row.password;
    let db_hash: String = String::from_utf8_lossy(db_hash_bytes).to_string();
    println!("real_db_hash: {}", &db_hash);

    let (_pre, hash) = split_db_hash(&db_hash);
    let verify = match utils::verify_password(_payload_passwd, &hash) {
        Ok(v) => v,
        Err(e) => {
            // 여기서 로그 찍기
            error!("password verify failed: {:?}", e);
            return Err(ApiError::InternalServerError);
        }
    };
    if verify {
        Ok(ApiResponse::JsonData(payload))
    } else {
        Err(ApiError::Unauthorized)
    }
}

pub fn users_router() -> Router<Arc<AppState>> { // 추후에 main_router에 머지할 꺼니까
    Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/login", post(post_login))
}