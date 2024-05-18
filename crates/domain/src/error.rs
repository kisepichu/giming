pub trait Error: std::error::Error {
    fn print_chain(&self);
}

impl<E: std::error::Error> Error for E {
    fn print_chain(&self) {
        eprintln!("{}", self);
        let mut current_error: &dyn std::error::Error = self;
        while let Some(cause) = current_error.source() {
            eprintln!("  caused by: {}", cause);
            current_error = cause;
        }
    }
}
