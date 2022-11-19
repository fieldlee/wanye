#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate cached;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate rbatis;

//配置
pub mod config;
//初始化
pub mod init;
//api
pub mod apis;
//middleware
pub mod middleware;
//services
pub mod services;
//utils
pub mod utils;
//models
pub mod models;
//crud
pub mod crud;
//controller
pub mod controllers;
//cache
pub mod cache;
// orm
pub mod orm;

use log::info;
use state::Container;

/*
    整个项目上下文ApplicationContext
    包括:
        ApplicationConfig 配置
        Database mongodb数据库
        Rbatis  mysql orm
        ServiceContext 服务上下文
        CasbinService 权限服务
*/

use init::init_config;
use init::init_log;
use init::init_database;
use init::init_service;
use config::config::ApplicationConfig;

pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();


/*初始化环境上下文*/
pub async fn init_context() {
    print_banner();
     //第一步加载配置
     init_config().await;
     //第二步加载日志
     init_log();
     info!("ConfigContext init complete");
     //第三步初始化数据源
     init_database().await;
     //第四步初始化cache
     init_service().await;
     info!("DataBase init complete");
    let commerce_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    info!(
        " - Local:   http://{}:{}",
        commerce_config
            .server()
            .host()
            .replace("0.0.0.0", "127.0.0.1"),
            commerce_config.server().port()
    );
}

fn print_banner() {
    let banner = r#"
     ____
    |      。   ———————     |                |     |        _____    ____
    |___   |   |            |                |     |       |        |
    |      |   |_______     |         —————— |     |       |_____   |____
    |      |   |            |         |      |     |       |        |
    |      |   |————————    |______   |_____ |     |____   |_____   |____
"#;
    println!("{}", banner);
}
