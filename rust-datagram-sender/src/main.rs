use std::env;
use std::io;
use std::net::UdpSocket;
use std::os::unix::net::UnixDatagram;

const UDP_PACKET_LIMIT_BYTES: u32 = 65_507;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <mode> <address/port or path> <size>", args[0]);
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid arguments",
        ));
    }

    let mode = &args[1];
    let target = &args[2];
    let size: usize = args[3].parse().expect("Size should be a number");

    let mut payload = vec![b' '; size];
    let start = b"BEGIN";
    let end = b"  END";
    let sequence = b"1234567890";

    // Construct payload
    for (i, &byte) in start.iter().enumerate().take(size) {
        payload[i] = byte;
    }
    for i in 5..size - 5 {
        payload[i] = sequence[(i - 5) % sequence.len()];
    }
    for (i, &byte) in end.iter().enumerate().take(size) {
        payload[size - 5 + i] = byte;
    }

    let address;

    let bytes_sent = match mode.as_str() {
        "udp" => {
            let socket = UdpSocket::bind("0.0.0.0:0")?;
            address = format!("127.0.0.1:{target}");
            match socket.send_to(&payload, address.clone()) {
                Ok(bytes_sent) => bytes_sent,
                Err(e) => {
                    eprintln!("Send failed with err: {}", e);
                    0_usize
                }
            }
        }
        "unix" => {
            let socket = UnixDatagram::unbound()?;
            address = target.to_string();
            socket.send_to(&payload, target)?
        }
        _ => {
            eprintln!("Invalid mode. Use 'udp' or 'unix'.");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode"));
        }
    };

    println!(
        "Sent datagram of size {} bytes to {}. Bytes sent: {}",
        size, address, bytes_sent
    );

    Ok(())
}
