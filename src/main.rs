use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut packet: Vec<u8> = Vec::new();
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    sock.connect("8.8.8.8").await?;
    //Header creation
    // ID random
    packet.extend_from_slice(&0x1234u16.to_be_bytes());
    //Flags will use the one for standard query
    packet.extend_from_slice(&0x0100u16.to_be_bytes());
    //QDCount
    packet.extend_from_slice(&1u16.to_be_bytes());
    //ANCount
    packet.extend_from_slice(&0u16.to_be_bytes());
    //NSCount
    packet.extend_from_slice(&0u16.to_be_bytes());
    //ARCount
    packet.extend_from_slice(&0u16.to_be_bytes());

    Ok(())
}
