use crate::APPLICATION_CONTEXT;
use crate::config::config::ApplicationConfig;
use tokio::fs::read_to_string;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::ZipPacker;
use std::time::Duration;
use crate::orm::init_rbatis;
use rbatis::rbatis::Rbatis;
use log::info;
use crate::cache::cache::CacheService;
use crate::services::user_service::UserService;
use crate::services::user_auth_service::UserAuthService;
//初始化配置信息
pub async fn init_config() {
    let content = read_to_string("application.yaml").await.unwrap();
    let mut config = ApplicationConfig::new(content.as_str());
    let mut list = match config.admin_auth_list_api().clone() {
        Some(e) => e,
        None => Vec::new(),
    };
    /*添加需要登录但是不需要权限的路由
     * 如果有额外的可以在application.yml中添加
     * admin_auth_list_api
     *  - XXXXXX
     *  - XXXXX
     * */
    list.push("/user/info".to_string());
    list.push("/dict/type/all".to_string());
  
    config.set_admin_auth_list_api(Some(list));
    
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}

pub fn init_log() {
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //create log dir
    std::fs::create_dir_all(&cassie_config.log_dir());
    //initialize fast log
    fast_log::init(
        Config::new()
            .console()
            .file_split(
                &cassie_config.log_dir(),
                str_to_temp_size(&cassie_config.log_temp_size()),
                str_to_rolling(&cassie_config.log_rolling_type()),
                ZipPacker {},
            )
            .level(log::LevelFilter::Info),
    )
    .unwrap();
}


fn str_to_temp_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn str_to_rolling(arg: &str) -> RollingType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}

fn str_to_log_level(arg: &str) -> log::Level {
    return match arg {
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        "trace" => log::Level::Trace,
        "info" => log::Level::Info,
        "debug" => log::Level::Debug,
        _ => log::Level::Info,
    };
}



pub async fn init_database() {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();

    let rbatis = init_rbatis(config).await;
    info!("link database success!{}", config.database_url());
    APPLICATION_CONTEXT.set::<Rbatis>(rbatis);

    // let mdb = init_mongodb(config).await;
    // APPLICATION_CONTEXT.set::<Database>(mdb);
    // info!("mongodb link database ({})...", config.mongodb_url());
}



pub async fn init_service() {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    APPLICATION_CONTEXT.set::<CacheService>(CacheService::new().unwrap());
    info!("CacheService init success!");
    // APPLICATION_CONTEXT.set::<SysAuthService>(SysAuthService::default());
    // info!("SysUserService init success!");
    // APPLICATION_CONTEXT.set::<SysUserService>(SysUserService::default());
    // info!("SysRoleService init success!");
    // APPLICATION_CONTEXT.set::<SysRoleService>(SysRoleService::default());
    // info!("SysMenuService init success!");
    // APPLICATION_CONTEXT.set::<SysMenuService>(SysMenuService::default());
    // info!("SysMenuService init success!");
    // APPLICATION_CONTEXT.set::<SysParamsService>(SysParamsService::default());
    // info!("SysParamsService init success!");
    // APPLICATION_CONTEXT.set::<SysDictTypeService>(SysDictTypeService::default());
    // info!("SysDictTypeService init success!");
    // APPLICATION_CONTEXT.set::<SysDictDataService>(SysDictDataService::default());
    // info!("SysDictDataService init success!");
    // APPLICATION_CONTEXT.set::<AsiGroupService>(AsiGroupService::default());
    // info!("AsiGroupService init success!");
    // APPLICATION_CONTEXT.set::<UploadService>(UploadService::new(config).unwrap());
    // APPLICATION_CONTEXT.set::<LogLoginService>(LogLoginService::default());
    // info!("LogLoginService init success!");
    // APPLICATION_CONTEXT.set::<LogOperationService>(LogOperationService::default());
    // info!("LogOperationService init success!");
    // APPLICATION_CONTEXT.set::<EventConfigService>(EventConfigService {});
    // info!("EventConfigService init success!");
    //apis  用户服务
    APPLICATION_CONTEXT.set::<UserService>(UserService::default());
    info!("UserService init success!");
    APPLICATION_CONTEXT.set::<UserAuthService>(UserAuthService::default());
    info!("UserAuthService init success!");
}
