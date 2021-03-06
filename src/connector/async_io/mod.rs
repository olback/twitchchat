use crate::connector::try_connect;
use crate::BoxedFuture;

type TcpStream = async_io::Async<std::net::TcpStream>;

mod non_tls;
pub use non_tls::*;

#[cfg(feature = "async-tls")]
mod tls;

#[cfg(feature = "async-tls")]
pub use tls::*;
