use axum::{response::IntoResponse, routing::post, Json, Router};
use crate::utils::error::Error;
use crate::utils::RespVO;
use crate::config::config::ApplicationConfig;
use crate::models::vo::{jwt::JWTToken, sign_in::ApiSignInVO, wx::WxSignInVo};
use rbatis::DateTimeNative;
use crate::models::UserType;
use crate::{
    services::user_service::UserService,
    crud::crud_service::CrudService,
    APPLICATION_CONTEXT,
    services::wechat_service::{binding_phone, wxapp_auth},
};

//小程序授权登录
pub async fn mp_auth(Json(sign): Json<WxSignInVo>) -> impl IntoResponse {
    match wxapp_auth(sign).await {
        Ok(uid) => {
            //根据用户信息 生成token
            let user_service = APPLICATION_CONTEXT.get::<UserService>();
            let user = user_service.get(uid.to_string()).await.unwrap();
            let mut jwt_token = JWTToken::default();
            jwt_token.set_id(uid);
            jwt_token.set_user_type(Some(UserType::Simple));
            jwt_token.set_username(user.account().clone().unwrap_or_default());
            jwt_token.set_from(crate::models::vo::jwt::Source::Wxapp);
            jwt_token.set_exp(DateTimeNative::now().timestamp_millis() as usize);

            let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
            //创建token
            match jwt_token.create_token(cassie_config.jwt_secret()) {
                Ok(token) => {
                    let mut result = ApiSignInVO::default();
                    result.set_user(Some(user));
                    result.set_access_token(token);
                    return RespVO::from(&result).resp_json();
                }
                Err(_) => {
                    return RespVO::<()>::from_error(&Error::from("获取用户访问token失败"))
                        .resp_json();
                }
            }
        }
        Err(e) => {
            return RespVO::<()>::from_error(&e).resp_json();
        }
    }
}
//授权获取小程序用户手机号 直接绑定
pub async fn auth_binding_phone(Json(sign): Json<WxSignInVo>) -> impl IntoResponse {
    match binding_phone(sign).await {
        Ok(s) => {
            return RespVO::from(&s).resp_json();
        }
        Err(e) => return RespVO::<()>::from_error(&e).resp_json(),
    }
}

//小程序支付回调
pub async fn notify() {
    todo!("待开发")
}

pub fn init_router() -> Router {
    Router::new()
        .route("/wechat/mp_auth", post(mp_auth))
        .route("/wechat/auth_bindind_phone", post(auth_binding_phone))
}
