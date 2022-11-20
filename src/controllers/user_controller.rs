use chrono::Days;
use axum::{response::IntoResponse,response::Response};
use crate::utils::RespVO;
use axum::extract::Path;
use axum::body::Body;
use crate::cache::cache::CacheService;
use crate::services::user_auth_service::UserAuthService;
use crate::services::user_service::UserService;
use axum::Json;
use crate::utils::error::Error;
use crate::models::dto::sign_in::{SignInDTO,CheckInDTO,CheckResultDTO,SignInByPhoneDTO,SendPhoneDTO};
use crate::models::dto::user_dto::UserDTO;
use validator::Validate;
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;
use crate::APPLICATION_CONTEXT;
use crate::utils::password_encoder::PasswordEncoder;
use rbatis::DateNative;
use crate::models::{LoginType,UserType,StatusType};
use base64::{encode, decode};
use crate::utils::string::random_code;

//获取用户信息
pub async fn get_user_info(Path(id): Path<String>) -> impl IntoResponse {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let user = user_service.get_user_by_id(id).await;
    return RespVO::from_result(&user).resp_json();
}

//用户登录
pub async fn user_login(Json(sign): Json<SignInDTO>) -> impl IntoResponse {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    let user_auth_service = APPLICATION_CONTEXT.get::<UserAuthService>();
    if let Err(e) = sign.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }
    if let Ok(code) = cache_service
        .get_string(&format!("_captch:uuid_{}", &sign.uuid().clone().unwrap()))
        .await
    {
        if !code.eq(&sign.vcode().clone().unwrap()) {
            return RespVO::<()>::from_error(&Error::E("验证码错误".to_string())).resp_json();
        }
    }
    let vo = user_auth_service.sign_in(&sign).await;

    return RespVO::from_result(&vo).resp_json();
}

//用户登录手机号
pub async fn user_login_phone(Json(sign): Json<SignInByPhoneDTO>) -> impl IntoResponse {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    let user_auth_service = APPLICATION_CONTEXT.get::<UserAuthService>();
    if let Err(e) = sign.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }
    if let Ok(code) = cache_service
        .get_string(&format!("_captch:phone_{}", &sign.phone().clone().unwrap()))
        .await
    {
        if !code.eq(&sign.code().clone().unwrap()) {
            return RespVO::<()>::from_error(&Error::E("验证码错误".to_string())).resp_json();
        }
    }
    let vo = user_auth_service.sign_in_phone(&sign).await;
    return RespVO::from_result(&vo).resp_json();
}

//用户注册
pub async fn user_register(Json(register): Json<UserDTO>) -> impl IntoResponse {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let password = PasswordEncoder::encode(&register.pwd().clone().unwrap().as_str());
    //password
    let mut m_register_info = register;
    m_register_info.set_pwd(Some(password));
    //超期时间
    let current = DateNative::now();
    let after_date = DateNative{inner:current.checked_add_days(Days::new(30)).unwrap()} ;
    m_register_info.set_expired_at(Some(after_date));
    //userType
    m_register_info.set_user_type(Some(UserType::Simple));
    m_register_info.set_login_type(Some(LoginType::PasswordLogin));
    m_register_info.set_status(Some(StatusType::Normal));

    //join_code
    if m_register_info.phone().is_some(){
        //phone
        let join_code = encode(m_register_info.phone().as_ref().unwrap());
        m_register_info.set_join_code(Some(join_code));
    }else{
        //account
        let join_code = encode(m_register_info.account().as_ref().unwrap());
        m_register_info.set_join_code(Some(join_code));
    }

    let resut = user_service.save_info(m_register_info).await;
    return RespVO::from_result(&resut).resp_json();
}

//check账号是否使用
pub async fn check_user_account(Json(check_info): Json<CheckInDTO>) -> impl IntoResponse {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let mut check_result = CheckResultDTO::new(false, false);
    if !check_info.account().is_none(){
        let user = user_service.get_user_by_account(check_info.account().as_ref().unwrap().to_string()).await;
        if user.is_ok() {
          check_result.set_account(true);
        }
    }

    if !check_info.phone().is_none(){
        let user =   user_service.get_user_by_phone(check_info.phone().as_ref().unwrap().to_string()).await;
        if user.is_ok() {
            check_result.set_phone(true);
          }
    }

    return RespVO::from_result(&Ok(check_result)).resp_json();
}

//更新用户
pub async fn update_user_info(Json(userinfo): Json<UserDTO>) -> impl IntoResponse {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
   
    let mut m_user_info = userinfo;
    let s_id = m_user_info.id().unwrap().to_string();
    let user = user_service.get_user_by_id(s_id).await;
    if user.is_ok(){
        if m_user_info.address().as_ref().unwrap().is_empty() {
            m_user_info.set_address(Some(user.as_ref().unwrap().address().as_ref().unwrap().to_string()));
        }
        if m_user_info.avatar().as_ref().unwrap().is_empty() {
            m_user_info.set_avatar(Some(user.as_ref().unwrap().avatar().as_ref().unwrap().to_string()));
        }
        if m_user_info.email().as_ref().unwrap().is_empty() {
            m_user_info.set_email(Some(user.as_ref().unwrap().email().as_ref().unwrap().to_string()));
        }
        if m_user_info.nickname().as_ref().unwrap().is_empty() {
            m_user_info.set_nickname(Some(user.as_ref().unwrap().nickname().as_ref().unwrap().to_string()));
        }
    }
    
    let resut = user_service.update_info(m_user_info).await;
    return RespVO::from_result(&resut).resp_json();
}

//captch 图片
pub async fn captcha_png(Path(uuid): Path<String>) -> Response<Body> {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    if uuid.is_empty() {
        return RespVO::<()>::from_error(&Error::from("uuid不能为空!")).resp_json();
    }
    let (captcha_str, png) = {
        let mut captcha = Captcha::new();
        captcha
            .add_chars(4)
            .apply_filter(Noise::new(0.1))
            .apply_filter(Wave::new(1.0, 10.0).horizontal())
            // .apply_filter(Wave::new(2.0, 20.0).vertical())
            .view(160, 60)
            .apply_filter(Dots::new(4));

        let png = captcha.as_png().unwrap();
        (captcha.chars_as_string().to_lowercase(), png)
    };

    let res = cache_service
        .set_string_ex(
            &format!("_captch:uuid_{}", uuid.clone()),
            captcha_str.as_str(),
            Some(std::time::Duration::from_secs(60 * 5)),
        )
        .await;
    println!("{:?}", res);
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Cache-Control", "no-cache")
        .header("Content-Type", "image/png")
        .body(Body::from(png))
        .unwrap()
}

//captch base64
pub async fn captcha_base64(Path(uuid): Path<String>) -> impl IntoResponse {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    if uuid.is_empty() {
        return RespVO::<()>::from_error(&Error::from("uuid不能为空!")).resp_json();
    }
    let (captcha_str, png) = {
        let mut captcha = Captcha::new();
        captcha
            .add_chars(4)
            .apply_filter(Noise::new(0.1))
            .apply_filter(Wave::new(1.0, 10.0).horizontal())
            // .apply_filter(Wave::new(2.0, 20.0).vertical())
            .view(160, 60)
            .apply_filter(Dots::new(4));

        let png = captcha.as_base64();
        (captcha.chars_as_string().to_lowercase(), png)
    };

    let res = cache_service
        .set_string_ex(
            &format!("_captch:uuid_{}", uuid.clone()),
            captcha_str.as_str(),
            Some(std::time::Duration::from_secs(60 * 5)),
        )
        .await;
    println!("{:?}", res);
    return RespVO::from(&png.unwrap()).resp_json();
}

//手机验证码
pub async fn send_phone_sms(Json(phoneinfo): Json<SendPhoneDTO>) -> impl IntoResponse {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    //验证
    if let Err(e) = phoneinfo.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }
    if let Ok(code) = cache_service
    .get_string(&format!("_captch:phone_{}", &phoneinfo.phone().clone().unwrap()))
    .await
    {
        if !code.eq(&phoneinfo.vcode().clone().unwrap()) {
            return RespVO::<()>::from_error(&Error::E("验证码错误".to_string())).resp_json();
        }
    }
    let res = cache_service
        .set_string_ex(
            &format!("_captch:phone_{}", phoneinfo.phone().as_ref().unwrap().clone()),
            random_code().as_str(),
            Some(std::time::Duration::from_secs(60 * 5)),
        )
        .await;
    println!("{:?}", res);
    

    return RespVO::from(&res).resp_json();
}