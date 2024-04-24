use serde::{Deserialize, Serialize};

pub struct OjToolsApi {}

#[derive(Serialize, Deserialize)]
pub struct LoginServiceResponse {
    #[serde(rename = "loggedIn")]
    pub logged_in: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OjToolsJson<T> {
    pub status: String,
    pub messages: Vec<String>,
    pub result: T,
}

impl OjToolsApi {
    pub fn login_service(
        &self,
        username: String,
        password: String,
        service_url: String,
    ) -> OjToolsJson<LoginServiceResponse> {
        println!(
            "login service: username={}, password={}, service_url={}",
            username, password, service_url
        );
        // TODO
        OjToolsJson {
            status: "ok".to_string(),
            messages: vec![],
            result: LoginServiceResponse { logged_in: true },
        }
    }
}
