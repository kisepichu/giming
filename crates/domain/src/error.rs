pub trait Error: std::error::Error {
    fn error_chain(&self) -> String;
}

impl<E: std::error::Error> Error for E {
    fn error_chain(&self) -> String {
        let mut res = String::new();
        res += &format!("{}", self);
        let mut current_error: &dyn std::error::Error = self;
        while let Some(cause) = current_error.source() {
            res += &format!("\n caused by: {}", cause);
            current_error = cause;
        }
        res
    }
}

#[derive(Debug)]
pub struct DummyDetailError;

impl DummyDetailError {
    pub fn new() -> Self {
        Self
    }
}

impl std::fmt::Display for DummyDetailError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dummy error for testing")
    }
}

impl std::error::Error for DummyDetailError {}

// custom Result implements unwrap_chain() to print error chain
pub trait ResultChain<T, E: Error> {
    fn unwrap_chain(self) -> T;
}

impl<T, E: Error> ResultChain<T, E> for Result<T, E> {
    fn unwrap_chain(self) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err.error_chain());
                panic!();
            }
        }
    }
}
