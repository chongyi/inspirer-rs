#[macro_export]
macro_rules! coded_error {
    ($err:ident ($code:expr) $msg:expr) => {
        #[derive(Debug, Clone)]
        pub struct $err;
        impl $crate::result::CodedError for $err {
            /// 获取错误代码
            fn error_code(&self) -> i16 { $code }
            /// 获取错误消息
            fn error_message(&self) -> &str { $msg }
        }

        impl StdError for $err {
            fn description(&self) -> &str {
                self.error_message()
            }
        }

        impl std::fmt::Display for $err {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{:?}", self)
            }
        }
    }
}