pub mod error;
pub mod online_judge;

use domain::error::Error;

use std::marker::PhantomData;

use online_judge::OnlineJudge;

use self::error::ServiceError;

pub struct InitArgs {
    pub contest_id: String,
}

pub struct Service<E: Error + 'static, O: OnlineJudge<E>> {
    online_judge: O,
    _phantom: PhantomData<E>,
}

impl<E: Error + 'static, O: OnlineJudge<E>> Service<E, O> {
    pub fn new(online_judge: O) -> Self {
        Self {
            online_judge,
            _phantom: PhantomData,
        }
    }
    pub fn login(&self, username: String, password: String) -> Result<(), Box<ServiceError<E>>> {
        self.online_judge.login(username, password)
    }
    pub fn init(&self, _contest_id: String) -> Result<(), Box<ServiceError<E>>> {
        // self.online_judge.get_contest 等使いコンテストディレクトリを作るロジックを書く
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use domain::error::DummyDetailError;

    use self::online_judge::MockOnlineJudge;

    use super::*;

    #[test]
    fn test_login() -> Result<(), String> {
        // login はそのまま受け渡すだけなのであまり意味はないが、小さい例としてテストを書く
        // test the minimal function login() as an example
        let online_judge = MockOnlineJudge::<DummyDetailError>::new();
        let mut service = Service::new(online_judge);

        // invalid username or password
        {
            service
                .online_judge
                .expect_login()
                .times(1)
                .returning(|_, _| {
                    Err(Box::new(ServiceError::LoginFailed(DummyDetailError::new())))
                });

            let username = "user".to_string();
            let password = "pass".to_string();
            let result = service.login(username, password);
            if let Err(e) = result {
                if let ServiceError::LoginFailed(_) = *e {
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
            service
                .online_judge
                .expect_login()
                .times(1)
                .returning(|_, _| Ok(()));
            let username = "user".to_string();
            let password = "pass".to_string();
            let result = service.login(username, password);
            result.map_err(|e| format!("Expected Ok, but got {:?}", e))?;
        }
        Ok(())
    }
}
