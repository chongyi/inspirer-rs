use error::ErrorDetail;
use actix_web::HttpRequest;

/// 错误消息体
///
/// 错误消息体最终会转换为对应的 JSON 格式。
#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorMessage<T> {
    /// 错误代码，`u16` 类型，默认为 65535，意为未知服务错误
    pub code: u16,
    /// 错误消息，`String` 类型。该字段简要描述错误信息
    pub msg: String,
    /// 错误详情
    pub body: Option<T>,
}

impl<T> ErrorMessage<T> {
    pub fn new(code: u16, msg: String, body: Option<T>) -> Self {
        ErrorMessage::<T> {
            code,
            msg,
            body,
        }
    }
}

impl Default for ErrorMessage<ErrorDetail> {
    fn default() -> Self {
        ErrorMessage::<ErrorDetail> {
            code: 65535,
            msg: "Unknown error.".into(),
            body: None
        }
    }
}

/// 分页列表数据结构
///
/// 页数可通过数据总数（`total`）与每页数据条数（`per_page`）得出。
#[derive(Deserialize,Serialize, Debug)]
pub struct PaginatedListMessage<T> {
    /// 列表数据
    pub list: Vec<T>,
    /// 数据总数
    pub total: i64,
    /// 当前页码
    pub page: i64,
    /// 每页数据条数
    pub per_page: i64,
}

#[derive(Copy, Clone)]
pub struct Pagination<T> {
    pub page: i64,
    pub per_page: i64,
    pub filter: Option<T>,
}

impl<T> Pagination<T> {
    pub const DEFAULT_PER_PAGE: i64 = 15;

    pub fn new(page: Option<i64>, per_page: Option<i64>, filter: Option<T>) -> Self {
        Self {
            page: match page {
                Some(v) => v,
                None => 1,
            },
            per_page: match per_page {
                Some(v) => v,
                None => Self::DEFAULT_PER_PAGE,
            },
            filter,
        }
    }

    pub fn from_request<S, R: AsRef<HttpRequest<S>>>(req: R) -> Self {
        let query = req.as_ref().query();

        let page = query.get("page")
            .map(|v| v.parse::<i64>().unwrap_or(1))
            .unwrap_or(1);
        let per_page = query.get("per_page")
            .map(|v| v.parse::<i64>().unwrap_or(Self::DEFAULT_PER_PAGE))
            .unwrap_or(Self::DEFAULT_PER_PAGE);

        Pagination::<T> {
            page,
            per_page,
            filter: None
        }
    }
}

#[derive(Copy, Clone)]
pub struct UpdateByID<T> {
    pub id: u32,
    pub update: T,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CreatedObjectIdMessage {
    pub id: u64,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct DeletedObjectMessage {
    pub count: u32,
}