use axum::{middleware::from_extractor, Router};
use crate::controllers::{init_need_auth_router,init_noneed_auth_router};
use crate::controllers::wx_controller;
use crate::middleware::auth_api;
//api
pub fn routers() -> Router {
    need_auth_routers().merge(noneed_auth_routers())
}
//需要权限认证的路由
pub fn need_auth_routers() -> Router {
    Router::new()
        .merge(init_need_auth_router())
        .layer(from_extractor::<auth_api::Auth>())
}

//不需要权限认证的路由
pub fn noneed_auth_routers() -> Router {
    Router::new()
    .merge(wx_controller::init_router())
    .merge(init_noneed_auth_router())
}



