use std::{
    env,
    io::{self, Write},
};

use domain::error::Error;
use rpassword::read_password;
use rustyline::{history::FileHistory, Editor};
use usecases::service_error::ServiceError;

use crate::detail_error::DetailError;

use super::{commands::LoginCommand, Shell};

impl Shell {
    pub fn login(&self, rl: &mut Editor<(), FileHistory>, args: LoginCommand) {
        let username = match get_username(rl, args.username) {
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
    rl: &mut Editor<(), FileHistory>,
    username: String,
) -> Result<String, ServiceError<DetailError>> {
    let username = if username.is_empty() {
        match env::var("ATCODER_USERNAME") {
            Ok(u) => u,
            Err(_) => {
                eprintln!(
                    "  tip: Set envvars for auto login. For more information, run 'help login'"
                );
                rl.readline("username: ").map_err(|e| {
                    ServiceError::LoginFailed(DetailError::Readline(
                        "failed to read username".to_string(),
                        e,
                    ))
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
                io::stdout().flush().map_err(|e| {
                    ServiceError::LoginFailed(DetailError::IO("flush stdout".to_string(), e))
                })?;
                read_password().map_err(|e| {
                    ServiceError::LoginFailed(DetailError::Custom(format!(
                        "failed to read password: {}",
                        e
                    )))
                })?
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
