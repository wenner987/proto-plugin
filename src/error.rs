use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum CyberError {
    #[error("模板读取错误: {0}")]
    TemplateError(String),

    #[error("Proto处理错误: {0}")]
    ProcessError(String),

    #[error("文件生成错误: {0}")]
    WriteError(String),

    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
}