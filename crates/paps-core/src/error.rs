use thiserror::Error;

/// PAPS 系统统一错误类型
#[derive(Error, Debug)]
pub enum PapsError {
    #[error("参数 '{0}' 不存在")]
    ParamNotFound(String),

    #[error("参数 '{0}' 的值 {1} 超出范围 [{2:?}, {3:?}]")]
    ValueOutOfRange(String, f64, f64, f64),

    #[error("JSON 解析失败: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("种子码无效: {0}")]
    InvalidSeed(String),

    #[error("关系类型无效: {0}")]
    InvalidRelationType(String),

    #[error("相变事件类型无效: {0}")]
    InvalidPhaseEvent(String),

    #[error("漂移参数无效: {0}")]
    InvalidDriftParam(String),

    #[error("序列化错误: {0}")]
    SerializationError(String),

    #[error("内部错误: {0}")]
    Internal(String),
}

/// PAPS 系统 Result 别名
pub type PapsResult<T> = Result<T, PapsError>;
