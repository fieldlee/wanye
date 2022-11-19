
use super::PAY_INDEX_VALUE_MAP;
use super::INDEX_VALUE_MAP;
/**
 *method:value_trans_index
 *desc: 淘宝指数转换
 *author:fieldlee
 *email:249608904@QQ.com
 */
pub fn value_trans_index(v:i32) -> i32{
    // (10*ln(x+1)+30)*x^0.5 
    let f_v = v as f32;
    ((((f_v+1.0).ln()*10.0)+30.0) * f_v.powf(0.5)) as i32
}

/**
 *method:pay_value_trans_index
 *desc: 淘宝支付指数转换
 *author:fieldlee
 *email:249608904@QQ.com
 */

pub fn pay_value_trans_index(v:i32) -> f32{
    match v {
        // 匹配区间
        3695..=i32::MAX => 100.0,
        i32::MIN..=0 => 0.0,
        _ => {
            match PAY_INDEX_VALUE_MAP.get(&v) {
                Some(value) => return *value,
                None => return 0.0,
            }
        },
    }
}


/**
 *method:get_value_by_index
 *desc: 淘宝支付指数转换
 *author:fieldlee
 *email:249608904@QQ.com
 */
pub fn get_value_by_index(v:i32) -> i32{
    for i in 0..=10 {
       let v2 = v + i;
       match INDEX_VALUE_MAP.get(&v2) {
        Some(value) => return *value,
        None => continue,
       }
    }
    0
}