#[macro_export]
macro_rules! coded_error {
    ($err:ident) => {
        #[derive(Debug, Clone)]
        pub struct $err;

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
    };

    ($err:ident ($code:expr) $msg:expr) => {
        coded_error!($err);

        impl $crate::error::CodedError for $err {
            /// 获取错误代码
            fn error_code(&self) -> i16 { $code }
            /// 获取错误消息
            fn error_message(&self) -> &str { $msg }
        }
    };

    ($err:ident ($code:expr) http($status:expr) $msg:expr) => {
        coded_error!($err);

        impl $crate::error::CodedError for $err {
            fn http_status(&self) -> $crate::error::StatusCode { $crate::error::StatusCode::from_u16($status).unwrap() }
            /// 获取错误代码
            fn error_code(&self) -> i16 { $code }
            /// 获取错误消息
            fn error_message(&self) -> &str { $msg }
        }
    }
}

#[macro_export]
macro_rules! map_actix_error {
    ($error:path, $code:expr) => {
        impl CodedError for ActixErrorWrapper<$error>
        {
            fn http_status(&self) -> StatusCode {
                self.err.error_response().status()
            }

            fn error_message(&self) -> &str {
                self.msg.as_str()
            }

            fn error_code(&self) -> i16 {
                $code
            }
        }
    };

    ($error:path { $($matcher:pat $(if $pred:expr)* => $result:expr),* }) => {
        impl CodedError for ActixErrorWrapper<$error>
        {
            fn http_status(&self) -> StatusCode {
                self.err.error_response().status()
            }

            fn error_message(&self) -> &str {
                self.msg.as_str()
            }

            fn error_code(&self) -> i16 {
                match &self.err {
                    $($matcher $(if $pred)* => $result),*
                }
            }
        }
    }
}