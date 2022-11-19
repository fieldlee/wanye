
use crate::models::dto::user_dto::UserDTO;
use serde::{Deserialize, Serialize};



/**
*struct:SignInVO
*desc:登录数据
*author:String
*email:348040933@qq.com
*/
#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SignInVO {
    user: Option<UserDTO>,
    access_token: String,
}

impl ToString for SignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ApiSignInVO {
    user: Option<UserDTO>,
    access_token: String,
}

impl ToString for ApiSignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
