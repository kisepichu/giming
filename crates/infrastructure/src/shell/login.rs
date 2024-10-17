use std::{
    env,
    io::{self, Write},
};

use domain::error::Error;
use rpassword::read_password;
use usecases::service_error::ServiceError;

use crate::detail_error::DetailError;

use super::{commands::LoginCommand, Shell};

impl Shell {
    pub fn login(
        &self,
        stdin_iter: &mut impl Iterator<Item = Result<String, std::io::Error>>,
        args: LoginCommand,
    ) {
        let username = match get_username(stdin_iter, args.username) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e.error_chain());
                return;
            }
        };
        let password = match get_password(&username, args.password) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e.error_chain());
                return;
            }
        };
        match self.controller.login(LoginCommand { username, password }) {
            Ok(_) => println!("login success"),
            Err(e) => {
                eprintln!("{}", e.error_chain());
                println!("login failed");
            }
        }
    }
}

fn get_username(
    stdin_iter: &mut impl Iterator<Item = Result<String, std::io::Error>>,
    username: String,
) -> Result<String, ServiceError<DetailError>> {
    let username = if username.is_empty() {
        match env::var("ATCODER_USERNAME") {
            Ok(u) => u,
            Err(_) => {
                eprintln!(
                    "  tip: Set envvars for auto login. For more information, run 'help login'"
                );
                print!("username: ");
                io::stdout().flush().unwrap();
                stdin_iter.next().unwrap().map_err(|e| {
                    ServiceError::LoginFailed(DetailError::InvalidInput(e.to_string()))
                })?
            }
        }
    } else {
        username
    };
    if username.is_empty() {
        Err(ServiceError::LoginFailed(DetailError::InvalidInput(
            "username is empty".to_string(),
        )))
    } else {
        Ok(username)
    }
}

fn get_password(username: &String, password: String) -> Result<String, ServiceError<DetailError>> {
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
        Err(ServiceError::LoginFailed(DetailError::InvalidInput(
            "password is empty".to_string(),
        )))
    } else {
        Ok(password)
    }
}
