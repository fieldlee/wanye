use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TaobaoInValueDTO {
    index: Option<i32>,
    values:Option<i32>,
}
impl TaobaoInValueDTO {
    pub fn new(index: i32, values:i32)->Self {
        TaobaoInValueDTO{
            index: Some(index),
            values: Some(values),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TaobaoInDTO {
    index: Option<i32>,
}



#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TaobaoPayInValueDTO {
    index: Option<i32>,
    values:Option<f32>,
}

impl TaobaoPayInValueDTO {
    pub fn new(index: i32, values:f32)->Self {
        TaobaoPayInValueDTO{
            index: Some(index),
            values: Some(values),
        }
    }
}