use axum::{routing::get,routing::post, Router};

pub mod user_controller;
pub mod wx_controller;

use user_controller::*;

pub fn init_need_auth_router() -> Router {
    Router::new().route("/user/:id", get(get_user_info))
}

pub fn init_noneed_auth_router() -> Router {
    Router::new()
    //-------------------------------------登录服务-------------------------------------------------------
    .route("/captcha/:uuid", get(captcha_base64))
    .route("/captcha/png/:uuid", get(captcha_png))
    .route("/login", post(user_login))
    .route("/register", post(user_register))
}