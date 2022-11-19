pub mod user_entity;
pub mod pagedata;


use rbatis::DateTimeNative;
/**
*struct:CommonField
*desc:所有表的公共字段 CRUD_SERVICE使用
*author:String
*email:348040933@qq.com
*/
#[derive(Clone, Debug)]
pub struct CommonField {
    pub id: Option<i64>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}