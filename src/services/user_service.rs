use crate::models::{
    dto::user_dto::{UserDTO},
    entitys::user_entity::{User},
    request::UserQuery,
};
use crate::utils::error::Result;
use rbatis::rbatis::Rbatis;
use rbatis::DateNative;
use crate::models::UserType;
use crate::utils::password_encoder::PasswordEncoder;
use crate::{crud::crud_service::CrudService, APPLICATION_CONTEXT};

pub struct UserService;
impl Default for UserService {
    fn default() -> Self {
        UserService {
        }
    }
}
impl UserService {
    //根据id查询用户
    pub async fn get_user_by_id(&self, id: String) -> Result<UserDTO> {
        let user_info = self.get(id.clone()).await.unwrap();
        return Ok(user_info);
    }

    //根据id删除用户
    pub async fn delete_user_by_id(&self, id: String) {
        let user_info = self.get(id.clone()).await.unwrap();
        self.del_by_column(User::id(), id.clone().as_str())
            .await;
    }

    //保存用户
    pub async fn save_info(&self, arg: UserDTO) -> Result<i64> {
        let mut entity: User = arg.into();
        /*保存到数据库*/
        if let Some(id) = entity.id {
            let user = self.get_user_by_id(id.to_string()).await;
            match user {
                Ok(_) => {
                    self.update_by_id(id.to_string(), &entity).await;
                    return Ok(id);
                },
                Err(err) => {
                    return Ok(0);
                }
            }
        } else {
            let id = self.save(&mut entity).await;
            return Ok(id.unwrap());
        }
    }

    //保存用户
    pub async fn update_info(&self, arg: UserDTO) -> Result<i64> {
        let entity: User = arg.into();
        /*保存到数据库*/
        if let Some(id) = entity.id {
            let user = self.get_user_by_id(id.to_string()).await;
            match user {
                Ok(_) => {
                    self.update_by_id(id.to_string(), &entity).await;
                    return Ok(id);
                },
                Err(err) => {
                    return Ok(0);
                }
            }
        } 
        return Ok(0);
    }
}

impl CrudService<User, UserDTO, UserQuery> for UserService {
    fn get_wrapper(arg: &UserQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(
        &self,
        common: crate::models::entitys::CommonField,
        data: &mut User,
    ) {

    }
}
