use std::{
    env,
    io::{self, Write},
};

use rpassword::read_password;
use usecases::service::{error::ServiceError, online_judge::OnlineJudge};

use crate::error::DetailError;

use super::{commands::LoginCommand, Shell};

impl<O: OnlineJudge<DetailError>> Shell<O> {
    pub fn login(
        &self,
        stdin_iter: &mut impl Iterator<Item = Result<String, std::io::Error>>,
        login_args: LoginCommand,
    ) -> Result<(), Box<ServiceError<DetailError>>> {
        let username = get_username(stdin_iter, login_args.username)?;
        let password = get_password(&username, login_args.password)?;
        self.controller.login(LoginCommand {
            username,
            password,
            online_judge: login_args.online_judge,
        })
    }
}

fn get_username(
    stdin_iter: &mut impl Iterator<Item = Result<String, std::io::Error>>,
    username: String,
) -> Result<String, Box<ServiceError<DetailError>>> {
    let username = if username.is_empty() {
        match env::var("ATCODER_USERNAME") {
            Ok(u) => u,
            Err(_) => {
                eprintln!(
                    "  - tips: Set envvars to avoid prompting. For more information, run 'help login'"
                );
                print!("username: ");
                io::stdout().flush().unwrap();
                stdin_iter.next().unwrap().map_err(|e| {
                    Box::new(ServiceError::LoginFailed(DetailError::InvalidInput(
                        e.to_string(),
                    )))
                })?
            }
        }
    } else {
        username
    };
    if username.is_empty() {
        Err(Box::new(ServiceError::LoginFailed(
            DetailError::InvalidInput("username is empty".to_string()),
        )))
    } else {
        Ok(username)
    }
}

fn get_password(
    username: &String,
    password: String,
) -> Result<String, Box<ServiceError<DetailError>>> {
    let password = if password.is_empty() {
        match env::var("ATCODER_PASSWORD") {
            Ok(p) => p,
            Err(_) => {
                // input from stdin
                print!("password for {}: ", username);
                io::stdout().flush().unwrap();
                read_password().unwrap()
            }
        }
    } else {
        password
    };
    if password.is_empty() {
        Err(Box::new(ServiceError::LoginFailed(
            DetailError::InvalidInput("password is empty".to_string()),
        )))
    } else {
        Ok(password)
    }
}
