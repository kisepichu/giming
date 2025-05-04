use usecases::service_error::ServiceError;

use crate::{detail_error::DetailError, external::atcoder_requester::AtcoderRequester};

use super::Atcoder;

impl<R: AtcoderRequester> Atcoder<R> {
    fn try_login(
        &self,
        username: &str,
        password: &str,
        cookies: Option<String>,
    ) -> Result<(), ServiceError<DetailError>> {
        || -> Result<(), DetailError> {
            let res = self.requester.login(username, password, cookies)?;

            let status = res.status();
            let url = res.url().to_string();
            println!("url: {}", url);
            let text = res.text().map_err(DetailError::Reqwest)?;
            if let Ok(whoamires) = self
                .whoami()
                .map_err(|e| DetailError::Internal("atcoder login".to_string(), Box::new(e)))
            {
                println!("username: {}", whoamires);
                Ok(())
            } else if text.contains("You have already signed in.") {
                println!("already signed in");
                Ok(())
            } else if text.contains("cookie is incorrect.") {
                Err(DetailError::InvalidCredentials("atcoder login"))
            } else if !status.is_success() {
                Err(DetailError::UnexpectedStatusCode("atcoder login", status))
            } else {
                println!("text: {}", text);
                Err(DetailError::UnexpectedResponse("atcoder login"))
            }
        }()
        .map_err(ServiceError::LoginFailed)
    }

    pub fn login(&self, username: &str, password: &str) -> Result<(), ServiceError<DetailError>> {
        self.try_login(username, password, None).or_else(|_| {
            let cookies = {
                let mut input = String::new();
                println!("Login failed. Try to login on browser and get REVEL_SESSION from DevTools>Application>Cookies.");
                println!("REVEL_SESSION: ");
                std::io::stdin()
                    .read_line(&mut input)
                    .map_err(|e|
                        ServiceError::LoginFailed(DetailError::IO("stdin".to_string(), e))
                    )?;
                Ok(format!("REVEL_FLASH=;REVEL_SESSION={}", input.trim()))
            }?;
            self.try_login(username, password, Some(cookies))
        })
    }
}
