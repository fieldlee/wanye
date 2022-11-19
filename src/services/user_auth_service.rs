use crate::models::{
    dto::user_dto::{ WechatUserDTO},
    entitys::user_entity::{User, WechatUser},
    request::UserQuery,
};
use rbatis::rbatis::Rbatis;
use crate::utils::error::Error;
use crate::utils::error::Result;
use crate::utils::password_encoder::PasswordEncoder;
use crate::config::config::ApplicationConfig;
use crate::models::dto::sign_in::SignInDTO;
use crate::models::vo::jwt::JWTToken;
use crate::models::vo::sign_in::SignInVO;
use crate::models::StatusType;
use rbatis::crud::CRUD;
use rbatis::DateTimeNative;
use crate::{crud::crud_service::CrudService, APPLICATION_CONTEXT};

pub struct WechatUserService;

impl WechatUserService {}

impl CrudService<WechatUser, WechatUserDTO, UserQuery> for WechatUserService {
    fn get_wrapper(arg: &UserQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let wrapper = rb.new_wrapper();
        rb.new_wrapper().do_if(arg.unionid().is_some(), |w| {
            w.eq(WechatUser::unionid(), &arg.unionid())
        })
    }

    fn set_save_common_fields(
        &self,
        common: crate::models::entitys::CommonField,
        data: &mut WechatUser,
    ) {
        
    }
}





/**
*struct:UserAuthService
*desc:用户权限服务  登录 错误重试
*author:String
*email:348040933@qq.com
*/
pub struct UserAuthService {}

impl Default for UserAuthService {
    fn default() -> Self {
        UserAuthService {}
    }
}

impl UserAuthService {
    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        /*验证码 验证*/
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let user: Option<User> = rb
            .fetch_by_wrapper(rb.new_wrapper().eq(User::account(), &arg.username()))
            .await?;
        let user = user.ok_or_else(|| {
            Error::from(format!(
                "账号:{} 不存在!",
                arg.username().clone().unwrap_or_default()
            ))
        })?;
        if !user.status.eq(&Some(StatusType::Normal)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;
        if !PasswordEncoder::verify(
            user.pwd
                .as_ref()
                .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
            arg.password()
                .as_ref()
                .ok_or_else(|| Error::from("密码不能为空"))?,
        ) {
            error = Some(Error::from("密码不正确!"));
        }
        if error.is_some() {
            return Err(error.unwrap());
        }
        let sign_in_vo = self.get_user_info(&user).await?;
        // let mut login_log = SysLogLoginDto::default();
        // login_log.set_operation(Some("0".to_string()));
        // login_log.set_user_agent(Some("admin".to_string()));
        // login_log.set_creator_name(arg.username().clone());
        // login_log.set_creator(user.id.clone());
        // let event = CassieEvent::LogLogin(login_log);
        // fire_event(event).await;
        return Ok(sign_in_vo);
    }

    /**
     *method:get_user_info_by_token
     *desc:根据token获取 暂时没用到
     *author:String
     *email:348040933@qq.com
     */
    pub async fn get_user_info_by_token(&self, token: &JWTToken) -> Result<SignInVO> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let user: Option<User> = rb
            .fetch_by_wrapper(rb.new_wrapper().eq(User::id(), &token.id()))
            .await?;
        let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", token.username())))?;
        return self.get_user_info(&user).await;
    }
    /**
     *method:get_user_info
     *desc:获取用户信息
     *author:String
     *email:348040933@qq.com
     */
    pub async fn get_user_info(&self, user: &User) -> Result<SignInVO> {
        let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
        //去除密码，增加安全性
        let mut user = user.clone();
        user.pwd = None;
        // let agency_code = user.agency_code.clone();
        let user_id = user
            .id
            .clone()
            .ok_or_else(|| Error::from("错误的用户数据，id为空!"))?;
        let mut sign_vo = SignInVO::default();
        sign_vo.set_user(Some(user.clone().into()));
        //提前查找所有权限，避免在各个函数方法中重复查找
        let mut jwt_token = JWTToken::default();
        jwt_token.set_id(user_id);
        jwt_token.set_user_type(user.user_type.clone());
        jwt_token.set_from(Default::default());
        jwt_token.set_username(user.account.clone().unwrap_or_default());
        jwt_token.set_exp(DateTimeNative::now().timestamp_millis() as usize);
        sign_vo.set_access_token(jwt_token.create_token(cassie_config.jwt_secret())?);
        return Ok(sign_vo);
    }
}
