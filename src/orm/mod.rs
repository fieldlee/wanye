use crate::config::config::ApplicationConfig;
use rbatis::rbatis::Rbatis;


///实例化 rbatis orm 连接池
pub async fn init_rbatis(commerce_config: &ApplicationConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if commerce_config.debug().eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    //连接数据库
    println!(
        "rbatis link database ({})...",
        commerce_config.database_url().clone()
    );
    rbatis
        .link(&commerce_config.database_url())
        .await
        .expect("rbatis link database fail!");
    println!("rbatis link database success!");

    return rbatis;
}

// use mongodb::{options::ClientOptions, Client, Database};

// pub async fn init_mongodb(commerce_config: &ApplicationConfig) -> Database {
//     let client_options = ClientOptions::parse(commerce_config.mongodb_url().clone().as_str())
//         .await
//         .expect(" mongodb link database fail!");
//     println!(
//         "mongodb link database ({})...",
//         commerce_config.mongodb_url().clone()
//     );
//     let client = Client::with_options(client_options).unwrap();
//     println!("mongodb link database success!");
//     let db = client.database("cassie");
//     db
// }
