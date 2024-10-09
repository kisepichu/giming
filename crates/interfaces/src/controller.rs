use domain::error::Error;
use usecases::service::{error::ServiceError, online_judge::OnlineJudge, Service};

pub mod input;
use input::LoginInput;

pub struct Controller<E: Error + 'static, O: OnlineJudge<E>> {
    pub service: Service<E, O>,
}

impl<E: Error + 'static, O: OnlineJudge<E>> Controller<E, O> {
    pub fn new(online_judge: O) -> Self {
        Self {
            service: Service::new(online_judge),
        }
    }
    pub fn login<T: LoginInput>(&self, args: T) -> Result<(), Box<ServiceError<E>>> {
        self.service.login(args.username(), args.password())
    }
}
