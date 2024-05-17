use usecases::service::{
    online_judge::{LoginArgs, OnlineJudge},
    Service,
};

pub mod input;
use input::LoginInput;

pub struct Controller<'o, E, O: OnlineJudge<E>> {
    pub service: Service<'o, E, O>,
}

impl<'o, E, O: OnlineJudge<E>> Controller<'o, E, O> {
    pub fn new(online_judge: &'o O) -> Self {
        Self {
            service: Service::new(online_judge),
        }
    }
    pub fn login<T: LoginInput>(&self, args: T) -> Result<(), E> {
        self.service.login(LoginArgs {
            username: args.username(),
            password: args.password(),
        })
    }
}
