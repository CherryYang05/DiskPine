/// 自定义错误处理
/// 

use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum HMSimError {
    ParseError,
    FileError,
    CommandError
}

impl Display for HMSimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HMSimError::ParseError => {
                write!(f, "单位转化错误")
            },
            HMSimError::FileError => {
                write!(f, "文件读写错误")
            },
            HMSimError::CommandError => {
                write!(f, "参数解析错误")
            }
        }
    }
}

impl From<std::io::Error> for HMSimError {
    fn from(_value: std::io::Error) -> Self {
        HMSimError::FileError
    }
}