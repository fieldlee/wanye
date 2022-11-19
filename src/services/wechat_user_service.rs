use crate::models::{
    dto::user_dto::{ WechatUserDTO},
    entitys::user_entity::{WechatUser},
    request::UserQuery,
};
use rbatis::rbatis::Rbatis;
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