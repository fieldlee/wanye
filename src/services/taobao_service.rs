
use crate::utils::error::Result;
use crate::models::dto::taobao_index::{TaobaoInValueDTO,TaobaoPayInValueDTO};
use crate::utils::taobao::{get_value_by_index,pay_value_trans_index};
pub struct TaobaoIndexService;

impl Default for TaobaoIndexService {
    fn default() -> Self {
        TaobaoIndexService {}
    }
}

impl TaobaoIndexService {
    pub async fn get_index_to_value(&self, index: i32) -> Result<TaobaoInValueDTO> {
        let mut taobao_info = TaobaoInValueDTO::new(index,0);
        let value =  get_value_by_index(index);
        taobao_info.set_values(Some(value));
        return Ok(taobao_info);
    }
}

pub struct TaobaoPayIndexService;
impl Default for TaobaoPayIndexService {
    fn default() -> Self {
        TaobaoPayIndexService {}
    }
}
impl TaobaoPayIndexService {
    pub async fn get_pay_index_to_value(&self, index: i32) -> Result<TaobaoPayInValueDTO> {
        let mut taobao_info = TaobaoPayInValueDTO::new(index,0.0);
        let value =  pay_value_trans_index(index);
        taobao_info.set_values(Some(value));
        return Ok(taobao_info);
    }
}