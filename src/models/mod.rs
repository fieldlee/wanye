pub mod entitys;
pub mod request;
pub mod vo;
pub mod dto;

use serde::{Deserialize, Serialize};


#[derive(Clone, Debug,Deserialize,Serialize,PartialEq,Eq)]
pub enum StatusType {
    Normal,
    Delete,
    Lose,
    Illegel
}
#[derive(Clone, Debug,Deserialize,Serialize,PartialEq,Eq)]
pub enum LoginType {
    PasswordLogin,
    PhoneLogin,
    EmailLogin,
    WechatLogin,
    MiniProgramLogin,
    PayLogin,
}
#[derive(Clone, Debug,Deserialize,Serialize,PartialEq,Eq)]
pub enum UserType {
    Simple,
    Month,
    ThreeMonth,
    SixMonth,
    Year,
    MonthPlus,
    ThreeMonthPlus,
    SixMonthPlus,
    YearPlus,
    MonthDoublePlus,
    ThreeMonthDoublePlus,
    SixMonthDoublePlus,
    YearDoublePlus,
}