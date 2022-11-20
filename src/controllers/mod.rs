use axum::{routing::get,routing::post, Router};

pub mod user_controller;
pub mod wx_controller;
pub mod taobao_controller;
use user_controller::*;
use taobao_controller::*;

pub fn init_need_auth_router() -> Router {
    Router::new()
    .route("/user/:id", get(get_user_info))
    .route("/update_user", post(update_user_info))
}

pub fn init_noneed_auth_router() -> Router {
    Router::new()
    //-------------------------------------登录服务-------------------------------------------------------
    .route("/captcha/:uuid", get(captcha_base64))
    .route("/captcha/png/:uuid", get(captcha_png))
    .route("/login", post(user_login))
    .route("/register", post(user_register))
    
    .route("/tranindex", post(get_index_value))
    .route("/tran_payindex", post(get_pay_index_value))
    .route("/check_keys", post(check_user_account))

    .route("/send_sms", post(send_phone_sms))
    .route("/login_phone", post(user_login_phone))
}