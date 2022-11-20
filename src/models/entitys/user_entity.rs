use rbatis::DateNative;
use crate::models::{StatusType,LoginType,UserType};

#[crud_table(table_name:user_info)]
#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub account: Option<String>,
    pub pwd: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub status: Option<StatusType>,
    pub user_type: Option<UserType>,
    pub pay_count: Option<f32>,
    pub login_type: Option<LoginType>,
    pub address: Option<String>,
    pub join_code: Option<String>,
    pub expired_at:Option<DateNative>,
}

impl_field_name_method!(User {
    id,
    account,
    pwd,
    nickname,
    avatar,
    phone,
    email,
    status,
    user_type,
    pay_count,
    address,
    join_code,
    login_type,
    expired_at,
});

#[crud_table(table_name:wechat_user)]
#[derive(Clone, Debug)]
pub struct WechatUser {
    pub id: Option<i64>,
    pub unionid: Option<String>,
    pub openid: Option<String>,
    pub routine_openid: Option<String>,
    pub nickname: Option<String>,
    pub headimgurl: Option<String>,
    pub sex: Option<u8>,
    pub city: Option<String>,
    pub language: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub remark: Option<i32>,
    pub groupid: Option<i32>,
    pub user_type: Option<UserType>,
    pub status_type: Option<StatusType>,
    pub session_key: Option<String>,
}
impl_field_name_method!(WechatUser {
    id,
    unionid,
    openid,
    routine_openid,
    nickname,
    headimgurl,
    sex,
    city,
    language,
    province,
    country,
    remark,
    groupid,
    user_type,
    status_type,
});
