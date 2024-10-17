use domain::error::Error;

use crate::service_error::ServiceError;

use super::Service;

impl<E: Error + 'static> Service<E> {
    pub fn whoami(&self) -> Result<String, ServiceError<E>> {
        self.online_judge.whoami()
    }
}

mod test {}
