pub mod error;
pub mod online_judge;

use domain::error::Error;

use std::marker::PhantomData;

use online_judge::{LoginArgs, OnlineJudge};

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
    pub fn login(&self, args: LoginArgs) -> Result<(), Box<ServiceError<E>>> {
        self.online_judge.login(args)
    }
    pub fn init(&self, _args: InitArgs) -> Result<(), Box<ServiceError<E>>> {
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
        let online_judge = MockOnlineJudge::<DummyDetailError>::new();
        let mut service = Service::new(online_judge);

        // invalid username or password
        {
            service
                .online_judge
                .expect_login()
                .returning(|_| Err(Box::new(ServiceError::LoginFailed(DummyDetailError::new()))));
            let args = LoginArgs {
                username: "user".to_string(),
                password: "pass".to_string(),
            };
            let result = service.login(args);
            if let Err(e) = result {
                if let ServiceError::LoginFailed(_) = *e {
                } else {
                    return Err(format!(
                        "Expected ServiceError::LoginFailed, but got {:?}",
                        e
                    ));
                }
            } else {
                return Err("Expected Err, but got Ok(())".to_string());
            }
        }
        // success
        {
            service.online_judge.expect_login().returning(|_| Ok(()));
            let args = LoginArgs {
                username: "user".to_string(),
                password: "pass".to_string(),
            };
            let result = service.login(args);
            result.map_err(|e| format!("Expected Ok(()), but got {:?}", e))?;
        }
        // already logged in
        {
            service.online_judge.expect_login().returning(|_| Ok(()));
            let args = LoginArgs {
                username: "".to_string(),
                password: "".to_string(),
            };
            let result = service.login(args);
            result.map_err(|e| format!("Expected Ok(()), but got {:?}", e))?;
        }
        Ok(())
    }
}
