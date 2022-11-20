use serde::{Deserialize, Serialize};
use rbatis::DateNative;
use crate::models::entitys::user_entity::{User, WechatUser};
use crate::models::{StatusType,LoginType,UserType};

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct UserDTO {
    id: Option<i64>,
    account: Option<String>,
    pwd: Option<String>,
    nickname: Option<String>,
    avatar: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    status: Option<StatusType>,
    user_type: Option<UserType>,
    pay_count: Option<f32>,
    address: Option<String>,
    join_code: Option<String>,
    login_type: Option<LoginType>,
    expired_at: Option<DateNative>,
}

impl Into<User> for UserDTO {
    fn into(self) -> User {
        User {
            id: self.id().clone(),
            account: self.account().clone(),
            pwd: self.pwd().clone(),
            nickname: self.nickname().clone(),
            avatar: self.avatar().clone(),
            phone: self.phone().clone(),
            email: self.email().clone(),
            status: self.status().clone(),
            user_type: self.user_type().clone(),
            expired_at: self.expired_at().clone(),
            pay_count: self.pay_count().clone(),
            address: self.address().clone(),
            join_code: self.join_code().clone(),
            login_type: self.login_type().clone(),
        }
    }
}

impl From<User> for UserDTO {
    fn from(arg: User) -> Self {
        Self {
            id: arg.id,
            account: arg.account,
            pwd: arg.pwd,
            nickname: arg.nickname,
            avatar: arg.avatar,
            phone: arg.phone,
            email: arg.email,
            status: arg.status,
            user_type: arg.user_type,
            pay_count: arg.pay_count,
            address: arg.address,
            join_code: arg.join_code,
            login_type: arg.login_type,
            expired_at: arg.expired_at,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct WechatUserDTO {
    id: Option<i64>,
    unionid: Option<String>,
    openid: Option<String>,
    routine_openid: Option<String>,
    nickname: Option<String>,
    headimgurl: Option<String>,
    sex: Option<u8>,
    city: Option<String>,
    language: Option<String>,
    province: Option<String>,
    country: Option<String>,
    remark: Option<i32>,
    groupid: Option<i32>,
    user_type: Option<UserType>,
    status_type: Option<StatusType>,
    session_key: Option<String>,
}

impl Into<WechatUser> for WechatUserDTO {
    fn into(self) -> WechatUser {
        WechatUser {
            id: self.id().clone(),
            unionid: self.unionid().clone(),
            openid: self.openid().clone(),
            routine_openid: self.routine_openid().clone(),
            nickname: self.nickname().clone(),
            headimgurl: self.headimgurl().clone(),
            sex: self.sex().clone(),
            city: self.city().clone(),
            language: self.language().clone(),
            province: self.province().clone(),
            country: self.country().clone(),
            remark: self.remark().clone(),
            groupid: self.groupid().clone(),
            user_type: self.user_type().clone(),
            status_type: self.status_type().clone(),
            session_key: self.session_key().clone(),
        }
    }
}

impl From<WechatUser> for WechatUserDTO {
    fn from(arg: WechatUser) -> Self {
        Self {
            id: arg.id,
            unionid: arg.unionid,
            openid: arg.openid,
            routine_openid: arg.routine_openid,
            nickname: arg.nickname,
            headimgurl: arg.headimgurl,
            sex: arg.sex,
            city: arg.city,
            language: arg.language,
            province: arg.province,
            country: arg.country,
            remark: arg.remark,
            groupid: arg.groupid,
            user_type: arg.user_type,
            status_type: arg.status_type,
            session_key: arg.session_key,
        }
    }
}
