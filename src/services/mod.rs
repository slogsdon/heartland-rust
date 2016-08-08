#[derive(Debug, Default)]
pub struct ServicesConfig {
    pub username: Option<&'static str>,
    pub password: Option<&'static str>,
    pub site_id: Option<&'static str>,
    pub license_id: Option<&'static str>,
    pub device_id: Option<&'static str>,
    pub secret_api_key: Option<&'static str>,
    pub version_number: Option<&'static str>,
    pub developer_id: Option<&'static str>,
}
