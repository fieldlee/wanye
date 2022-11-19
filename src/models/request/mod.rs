use serde::{Deserialize, Serialize};
use crate::models::vo::jwt::{JWTToken, Source};
use crate::models::UserType;
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct RequestModel {
    uid: i64,
    user_type: Option<UserType>,
    username: String,
    agency_code: String,
    product_code: String,
    path: String,
    from: Source,
}

impl RequestModel {
    pub fn new(data: JWTToken, path: String) -> Self {
        let mut model = RequestModel::default();
        model.set_uid(data.id().clone());
        model.set_user_type(data.user_type().clone());
        model.set_username(data.username().clone());
        model.set_path(path);
        model.set_from(data.from().clone());
        model
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct UserQuery {
    unionid: Option<String>,
    routine_openid: Option<String>,
}