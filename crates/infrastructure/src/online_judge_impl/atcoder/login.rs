use usecases::service_error::ServiceError;

use crate::{
    detail_error::DetailError,
    external::atcoder_requester::{AtcoderRequester, atcoder_requester_impl::HOME_URL},
};

use super::Atcoder;

impl<R: AtcoderRequester> Atcoder<R> {
    pub(crate) fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<(), ServiceError<DetailError>> {
        || -> Result<(), DetailError> {
            let res = self.requester.login(&username, &password)?;

            let status = res.status();
            let url = res.url().to_string();
            let text = res.text().map_err(DetailError::Reqwest)?;
            if url.contains(HOME_URL) {
                println!(
                    "username: {}",
                    self.whoami().map_err(|e| DetailError::Internal(
                        "atcoder login".to_string(),
                        Box::new(e)
                    ))?
                );
                Ok(())
            } else if text.contains("You have already signed in.") {
                println!("already signed in");
                Ok(())
            } else if text.contains("Username or Password is incorrect.") {
                Err(DetailError::InvalidCredentials("atcoder login"))
            } else if !status.is_success() {
                Err(DetailError::UnexpectedStatusCode("atcoder login", status))
            } else {
                Err(DetailError::UnexpectedResponse("atcoder login"))
            }
        }()
        .map_err(ServiceError::LoginFailed)
    }
}
