use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut packet: Vec<u8> = Vec::new();
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    sock.connect("8.8.8.8").await?;

    Ok(())
}
