mod pdfmonkey;
mod postgres;
mod sendinblue;
mod stripe;

pub use self::pdfmonkey::*;
pub use self::postgres::*;
pub use self::sendinblue::*;
pub use self::stripe::*;

pub trait Provider {
    fn init() -> Self;
}
