use core::fmt;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
};

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
pub use error_code_derive::ToErrorInfo;

pub struct ErrorInfo<T> {
    pub app_code: T,        // could be HTTP 400 bad request
    pub code: &'static str, // something like "01E739"
    pub hash: String,
    pub client_msg: &'static str,
    pub server_msg: String,
}

pub trait ToErrorInfo {
    type T: FromStr;
    fn to_error_info(&self) -> ErrorInfo<Self::T>;
}

impl<T> ErrorInfo<T>
where
    T: FromStr,
    //<T as FromStr>::Err：
    // 这是访问 T 的 FromStr 实现中定义的 Err 类型的语法。
    // 类似于“把 T 当作 FromStr 来看，然后取它的 Err 类型”。
    //“类型 T 必须实现 FromStr trait，并且 FromStr::Err（即 T 的解析错误类型）必须实现 Debug trait。”
    <T as FromStr>::Err: fmt::Debug,
{
    pub fn new(
        app_code: &str,
        code: &'static str,
        client_msg: &'static str,
        server_msg: impl fmt::Display,
    ) -> Self {
        let server_msg = server_msg.to_string();
        let mut hasher = DefaultHasher::new();
        server_msg.hash(&mut hasher);
        let hash = hasher.finish();
        let hash = BASE64_URL_SAFE_NO_PAD.encode(hash.to_le_bytes());
        Self {
            app_code: T::from_str(app_code).expect("can not parse"),
            code,
            hash,
            client_msg,
            server_msg: server_msg.to_string(),
        }
    }
}

impl<T> ErrorInfo<T> {
    pub fn client_msg(&self) -> &str {
        if self.client_msg.is_empty() {
            &self.server_msg
        } else {
            self.client_msg
        }
    }
}

impl<T> fmt::Display for ErrorInfo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}] {}", self.code, self.hash, self.client_msg())
    }
}

impl<T> fmt::Debug for ErrorInfo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}], {}", self.code, self.hash, self.server_msg)
    }
}
