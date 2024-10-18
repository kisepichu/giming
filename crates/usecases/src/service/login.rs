use domain::error::Error;

use crate::service_error::ServiceError;

use super::Service;

impl<E: Error + 'static> Service<E> {
    pub fn login(&self, username: String, password: String) -> Result<(), ServiceError<E>> {
        self.online_judge.login(username, password)
    }
}

#[cfg(test)]
mod tests {
    use domain::error::DummyDetailError;

    use crate::{
        config::{Config, MockConfig},
        online_judge::MockOnlineJudge,
        repository::{contest_repository::MockWorkspaceRepository, MockRepository},
        service::Service,
        service_error::ServiceError,
    };

    #[test]
    fn test_login() -> Result<(), String> {
        // login はそのまま受け渡すだけなのであまり意味はないが、小さい例としてテストを書く
        // test the minimal function login() as an example

        // invalid username or password
        {
            let config = MockConfig::new();
            let mut online_judge = MockOnlineJudge::<DummyDetailError>::new();
            online_judge
                .expect_login()
                .times(1)
                .returning(|_, _| Err(ServiceError::LoginFailed(DummyDetailError::new())));
            let contest_repository = MockWorkspaceRepository::<DummyDetailError>::new();
            let repository = MockRepository::new(Box::new(config), Box::new(contest_repository));
            let service = Service::new(
                Box::new(online_judge),
                Box::new(repository),
                "abc375".to_string(),
            );

            let username = "user".to_string();
            let password = "pass".to_string();
            let result = service.login(username, password);
            if let Err(e) = result {
                if let ServiceError::LoginFailed(_) = e {
                } else {
                    return Err(format!(
                        "Expected ServiceError::LoginFailed, but got {:?}",
                        e
                    ));
                }
            } else {
                return Err(format!("Expected Err, but got {:?}", result));
            }
        }
        // success
        {
            let config = MockConfig::new();
            let mut online_judge = MockOnlineJudge::<DummyDetailError>::new();
            online_judge
                .expect_login()
                .times(1)
                .returning(|_, _| Ok(()));
            let contest_repository = MockWorkspaceRepository::<DummyDetailError>::new();
            let repository = MockRepository::new(Box::new(config), Box::new(contest_repository));
            let service = Service::new(
                Box::new(online_judge),
                Box::new(repository),
                "abc375".to_string(),
            );

            let username = "user".to_string();
            let password = "pass".to_string();
            let result = service.login(username, password);
            result.map_err(|e| format!("Expected Ok, but got {:?}", e))?;
        }
        Ok(())
    }
}
