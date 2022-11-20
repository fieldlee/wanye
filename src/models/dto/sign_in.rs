use validator_derive::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SignInDTO {
    #[validate(required)]
    #[validate(length(min = 5, message = "账号最少5个字符"))]
    username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 6, message = "密码最少6个字符"))]
    password: Option<String>,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    #[validate(required)]
    #[validate(length(equal = 4, message = "验证码必须为4位"))]
    vcode: Option<String>,
    #[validate(required)]
    uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SignInByPhoneDTO {
    #[validate(required)]
    #[validate(length(min = 11, message = "账号最少11个字符"))]
    phone: Option<String>,
    #[validate(required)]
    #[validate(length(equal = 4, message = "验证码必须为4位"))]
    code: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SendPhoneDTO {
    #[validate(required)]
    #[validate(length(min = 11, message = "账号最少11个字符"))]
    phone: Option<String>,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    #[validate(required)]
    #[validate(length(equal = 4, message = "验证码必须为4位"))]
    vcode: Option<String>,
    #[validate(required)]
    uuid: Option<String>,
}

/// 验证码
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CatpchaDTO {
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct CheckInDTO {
    #[validate(required)]
    #[validate(length(min = 5, message = "账号最少5个字符"))]
    account: Option<String>,
    #[validate(required)]
    #[validate(length(min = 11, message = "手机号最少11个字符"))]
    phone: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct CheckResultDTO {
    account: bool,
    phone: bool,
}
impl CheckResultDTO{
    pub fn new(account: bool, phone: bool) -> Self{
        CheckResultDTO{
            account: account,
            phone: phone,
        }
    }
}