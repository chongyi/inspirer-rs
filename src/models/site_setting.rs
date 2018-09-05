#[derive(Deserialize, Serialize, Default)]
pub struct SiteSetting {
    pub site: SiteMetaData
}

#[derive(Deserialize, Serialize)]
pub struct SiteMetaData {
    pub title: String,
    pub home: String,
    pub registration_record: Option<String>,
}

impl Default for SiteMetaData {
    fn default() -> Self {
        SiteMetaData {
            title: String::from("Inspirer"),
            home: String::from("www.insp.top"),
            registration_record: None
        }
    }
}