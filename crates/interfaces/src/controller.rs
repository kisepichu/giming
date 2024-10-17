use domain::error::Error;
use usecases::{online_judge::OnlineJudge, service::Service, service_error::ServiceError};

pub mod input;
use input::LoginInput;

use self::input::{InitInput, WhoamiInput};

pub struct Controller<E: Error + 'static> {
    service: Service<E>,
}

impl<E: Error + 'static> Controller<E> {
    pub fn new(oj: Box<dyn OnlineJudge<E>>, contest_id: String) -> Self {
        Self {
            service: Service::new(oj, contest_id),
        }
    }
    pub fn online_judge_name(&self) -> &str {
        self.service.online_judge_name()
    }
    pub fn contest_id(&self) -> String {
        self.service.contest_id()
    }
    pub fn whoami<T: WhoamiInput>(&self, _args: T) -> Result<String, ServiceError<E>> {
        self.service.whoami()
    }
    pub fn login<T: LoginInput>(&self, args: T) -> Result<(), ServiceError<E>> {
        self.service.login(args.username(), args.password())
    }
    pub fn init<T: InitInput>(
        &mut self,
        args: T,
        oj_switch: Option<Box<dyn OnlineJudge<E>>>,
    ) -> Result<(), ServiceError<E>> {
        self.service.init(args.contest_id(), oj_switch)
    }
}
