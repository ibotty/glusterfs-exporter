use std::error::Error;

pub type GEError = Box<Error>;

#[macro_export]
macro_rules! err {
    ($($arg: tt)*) => (Err(From::from(format!($($arg)*))));
}
