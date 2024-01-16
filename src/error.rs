/// 自定义错误处理
/// 

use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum HMSimError {
    ParseError
}

impl Display for HMSimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HMSimError::ParseError => {
                write!(f, "单位转化错误")
            }
        }
    }
}