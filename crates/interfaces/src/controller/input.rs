pub trait ExitInput {
    fn code(&self) -> i32;
}

pub trait LoginInput {
    fn username(&self) -> String;
    fn password(&self) -> String;
}

pub trait InitInput {
    fn contest_id(&self) -> String;
}
