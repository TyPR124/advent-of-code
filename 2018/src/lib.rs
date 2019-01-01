#[derive(Debug, Clone)]
pub struct Error(String);
pub type Result<T> = std::result::Result<T, Error>;
impl Error {
    pub fn new(s: String) -> Error {
        Error(s)
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Error {
        Error(format!("FromUtf8Error: {:?}", e))
    }
}
#[macro_export]
macro_rules! err {
    ($($ex:expr),*) => {
        Err(Error::new(format!($($ex),*)));
    };
}