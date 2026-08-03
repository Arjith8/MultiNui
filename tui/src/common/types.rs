#[derive(Debug)]
pub enum ErrorLevel {
    INFO,
    WARNING,
    FATAL
}

#[derive(Debug)]
pub struct Error {
    pub text: String,
    pub level: ErrorLevel
}


