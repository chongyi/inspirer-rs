#[derive(Deserialize, Serialize, Default)]
pub struct SiteSetting {
    pub site: SiteMetaData
}

#[derive(Deserialize, Serialize)]
pub struct SiteMetaData {
    pub title: String,
    pub registration_record: Option<String>,
}

impl Default for SiteMetaData {
    fn default() -> Self {
        SiteMetaData {
            title: String::from("Inspirer"),
            registration_record: None
        }
    }
}