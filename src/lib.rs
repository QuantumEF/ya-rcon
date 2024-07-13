use std::net::TcpStream;

pub use client::RCONClient;

pub mod client;
pub mod packet;

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
