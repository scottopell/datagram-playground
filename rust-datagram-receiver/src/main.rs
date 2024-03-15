use std::env;
use std::io;
use std::net::UdpSocket;
use std::os::unix::net::UnixDatagram;

const UDP_PACKET_LIMIT_BYTES: usize = 65_507;
const MAX_PACKET_SIZE: usize = UDP_PACKET_LIMIT_BYTES * 10;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <mode> <address/port or path>", args[0]);
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid arguments",
        ));
    }

    match args[1].as_str() {
        "udp" => {
            let address = format!("0.0.0.0:{}", args[2]);
            let socket = UdpSocket::bind(&address)?;
            println!("UDP server listening on {}", address);
            udp_server(socket)?;
        }
        "unix" => {
            let path = &args[2];
            let _ = std::fs::remove_file(path); // Ignore error
            let socket = UnixDatagram::bind(path)?;
            println!("Unix domain server listening on {}", path);
            unix_server(socket)?;
        }
        _ => {
            eprintln!("Invalid mode. Use 'udp' or 'unix'.");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode"));
        }
    }

    Ok(())
}

fn udp_server(socket: UdpSocket) -> io::Result<()> {
    let mut buf = [0; MAX_PACKET_SIZE];
    loop {
        let (amt, _src) = socket.recv_from(&mut buf)?;
        handle_packet(&buf[..amt]);
    }
}

fn unix_server(socket: UnixDatagram) -> io::Result<()> {
    let mut buf = [0; MAX_PACKET_SIZE];
    loop {
        let amt = socket.recv(&mut buf)?;
        handle_packet(&buf[..amt]);
    }
}

fn handle_packet(packet: &[u8]) {
    let start = String::from_utf8_lossy(&packet[..5.min(packet.len())]);
    let end = String::from_utf8_lossy(&packet[(packet.len().saturating_sub(5))..]);
    println!("Length: {}, Start: {}, End: {}", packet.len(), start, end);
}
