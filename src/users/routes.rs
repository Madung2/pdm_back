use axum::{Router, routing::{get, post}, Json};
use serde::{Serialize, Deserialize};
use utils::utils;
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

async fn post_login(Json(payload): Json<LoginModel>) -> Json<Vec<LoginModel>> {
    let payload_user_id = payload.user_id;
    let payload_passwd = payload.passwd;
    let mock_pass = "pass".to_string()
    // let mock_hash = "3|$argon2id$v=19$m=65536,t=4,p=1$MFJCSVBOeGY1eVhKQnBsTw$g6j9pDVcQ1EDF4tK+EWUA2SsRsLlv0voa6Hpm3R8/Gk".to_string();
    // let verify = utils.verify_password()
    let verify = payload_passwd == mock_pass
    let this_user = vec![
        LoginModel { user_id: payload_user_id, passwd: payload_passwd },
        LoginModel { user_id: "devLovers2".to_string(), passwd: "hash2".to_string() },
    ];
    if verify {
        Json(this_user)
    } else {
        
    }
}

pub fn users_router() -> Router { // 추후에 main_router에 머지할 꺼니까 
    Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/login", post(post_login))
}