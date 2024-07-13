#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

use std::net::TcpStream;

pub use client::RCONClient;

#[allow(missing_docs)]
pub mod client;
#[allow(missing_docs)]
pub mod packet;

/// A simple RCON client using the `TcpStream` from the standard library.
///
/// # Example
/// ```
/// use ya_rcon::simple_tcp_client;
/// // You should actually handle the error in practice.
/// let client = simple_tcp_client("127.0.0.1:27015", "password".to_string()).unwrap();
/// ```
pub fn simple_tcp_client(
    addr: impl std::net::ToSocketAddrs,
    password: String,
) -> std::result::Result<
    client::RCONClient<std::net::TcpStream, std::ops::RangeFrom<u32>>,
    std::io::Error,
> {
    let stream = TcpStream::connect(addr)?;
    Ok(RCONClient::new(stream, 0.., password)?)
}
