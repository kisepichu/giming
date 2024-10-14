use domain::error::Error;
use usecases::service::{error::ServiceError, online_judge::OnlineJudge, Service};

pub mod input;
use input::LoginInput;

use self::input::InitInput;

pub struct Controller<E: Error + 'static> {
    pub service: Service<E>,
}

impl<E: Error + 'static> Controller<E> {
    pub fn new(oj: Box<dyn OnlineJudge<E>>) -> Self {
        Self {
            service: Service::new(oj),
        }
    }
    pub fn login<T: LoginInput>(&self, args: T) -> Result<(), Box<ServiceError<E>>> {
        self.service.login(args.username(), args.password())
    }
    pub fn init<T: InitInput>(&self, args: T) -> Result<(), Box<ServiceError<E>>> {
        self.service.init(args.contest_id())
    }
}
