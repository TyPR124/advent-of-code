#[derive(Debug, Clone)]
pub struct Error(String);
pub type Result<T> = std::result::Result<T, Error>;
impl Error {
    pub fn new(s: String) -> Error {
        Error(s)
    }
}

// impl From<std::string::FromUtf8Error> for Error {
//     fn from(e: std::string::FromUtf8Error) -> Error {
//         Error(format!("FromUtf8Error: {:?}", e))
//     }
// }
// impl From<std::num::ParseIntError> for Error {
//     fn from(e: std::num::ParseIntError) -> Error {
//         Error(format!("ParseIntError: {:?}", e))
//     }
// }
macro_rules! impl_froms_for_error {
    ($($from:ty),*) => {
        $(
            impl From<$from> for Error {
                fn from(e: $from) -> Error {
                    Error(format!(concat!(stringify!($from), ": {:?}"), e))
                }
            }
        )*
    };
}

impl_froms_for_error!(
    std::string::FromUtf8Error,
    std::num::ParseIntError
    //std::option::NoneError
);
#[macro_export]
macro_rules! err {
    ($($ex:expr),*) => {
        Error::new(format!($($ex),*));
    };
}