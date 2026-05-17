mod parser;
use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut packet: Vec<u8> = Vec::new();
    let mut buf = [0u8; 512];
    let mut pos = 12;
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    sock.connect("8.8.8.8:53").await?;
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
    //Start pushing the question
    // Start with size of the domain name
    packet.push(6);
    packet.extend_from_slice(b"google");
    //Size of the extension?
    packet.push(3);
    packet.extend_from_slice(b"com");
    packet.push(0);

    //QType
    packet.extend_from_slice(&1u16.to_be_bytes());
    //QClass
    packet.extend_from_slice(&1u16.to_be_bytes());

    sock.send(&packet).await?;
    let n = sock.recv(&mut buf).await?;
    println!("{:02x?}", &buf[..n]);
    Ok(())
}
