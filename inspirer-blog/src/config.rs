#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub listen: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            listen: "0.0.0.0:8006".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationConfig {
    /// App 名称
    pub name: String,
    /// App 域名
    pub domain: String,
    /// 用于 Token 等加密使用的字符串
    pub secret: String,
    /// token 过期时间长度，单位：秒
    pub token_lifetime: i64,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        ApplicationConfig {
            name: "Inspirer".into(),
            domain: "blog.insp.top".into(),
            secret: "secret".into(),
            token_lifetime: 28800,
        }
    }
}