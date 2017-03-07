#[macro_use]
extern crate slog;
extern crate rustun;
extern crate fibers;
extern crate futures;
#[macro_use]
extern crate trackable;
extern crate handy_async;

pub use error::{Error, ErrorKind};
pub use method::Method;

pub mod rfc5766;

mod error;
mod method;

pub type Result<T> = ::std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}