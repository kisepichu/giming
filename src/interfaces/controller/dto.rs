use clap::Parser;

// TODO: clap を知っている部分は infrastructure に書くべき
#[derive(Parser, Debug)]
pub struct ExitArgsDTO {
    #[clap(default_value = "0")]
    pub code: i32,
}

#[derive(Parser, Debug)]
pub struct LoginArgsDTO {
    #[clap(default_value = "")]
    pub username: String,
    #[clap(default_value = "")]
    pub password: String,
}
