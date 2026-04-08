use simple_dns::{Packet, PacketFlag, RCODE};
use std::{collections::HashSet, net::UdpSocket};

const UPSTREAM: &str = "8.8.8.8:53";
const LISTEN: &str = "0.0.0.0:53";
const BUF_SIZE: usize = 512;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(LISTEN)?;
    println!("Listening on {LISTEN}");

    let mut buf = [0u8; BUF_SIZE];

    let blacklist = HashSet::from(["google.com".to_string()]);

    loop {
        let (len, src) = socket.recv_from(&mut buf)?;
        let raw = &buf[..len];

        let query = match Packet::parse(raw) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to parse DNS query from {src}: {e}");
                continue;
            }
        };

        let domain = query
            .questions
            .first()
            .map(|q| q.qname.to_string())
            .unwrap_or_else(|| "<no question>".to_string());

        println!("{src} -> {domain}");

        if blacklist.contains(&domain) {
            println!("blocked!");
            if let Ok(mut reply) = Packet::parse(raw) {
                reply.set_flags(PacketFlag::RESPONSE);
                *reply.rcode_mut() = RCODE::NameError;
                if let Ok(bytes) = reply.build_bytes_vec() {
                    socket.send_to(&bytes, src)?;
                }
            }
            continue;
        }

        // Forward to upstream and relay response back
        match forward(raw) {
            Ok(response) => {
                socket.send_to(&response, src)?;
            }
            Err(e) => {
                eprintln!("Upstream failed for {domain}: {e}");
                // Send SERVFAIL back to client
                if let Ok(mut reply) = Packet::parse(raw) {
                    reply.set_flags(PacketFlag::RESPONSE);
                    *reply.rcode_mut() = RCODE::ServerFailure;
                    if let Ok(bytes) = reply.build_bytes_vec() {
                        socket.send_to(&bytes, src)?;
                    }
                }
            }
        }
    }
}

fn forward(query: &[u8]) -> std::io::Result<Vec<u8>> {
    let upstream = UdpSocket::bind("0.0.0.0:0")?;
    upstream.connect(UPSTREAM)?;
    upstream.send(query)?;

    let mut buf = [0u8; 4096];
    let len = upstream.recv(&mut buf)?;
    Ok(buf[..len].to_vec())
}
