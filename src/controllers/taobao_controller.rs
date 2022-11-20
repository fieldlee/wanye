use axum::Json;
use crate::models::{dto::taobao_index::TaobaoInDTO};
use crate::services::taobao_service::{TaobaoIndexService,TaobaoPayIndexService};
use crate::APPLICATION_CONTEXT;
use axum::response::IntoResponse;
use crate::utils::RespVO;
//用户指数转换
pub async fn get_index_value(Json(index_value): Json<TaobaoInDTO>) -> impl IntoResponse {
    let taobao_service = APPLICATION_CONTEXT.get::<TaobaoIndexService>();
   
    // taobao_service
    let result = taobao_service.get_index_to_value(index_value.index().unwrap()).await;

    return RespVO::from_result(&result).resp_json();
}
//用户支付指数转换
pub async fn get_pay_index_value(Json(index_value): Json<TaobaoInDTO>) -> impl IntoResponse {
    let taobao_service = APPLICATION_CONTEXT.get::<TaobaoPayIndexService>();
   
    // taobao_service
    let result = taobao_service.get_pay_index_to_value(index_value.index().unwrap()).await;

    return RespVO::from_result(&result).resp_json();
}