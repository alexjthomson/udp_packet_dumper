use std::net::{UdpSocket, SocketAddr, IpAddr};
use std::fs;
use std::time::SystemTime;

/// Size of the [`u8`] buffer used to read UDP packets.
const BUFFER_SIZE: usize = 65536; // 2^16 (max UDP packet size is 65527)

fn main() {
    let ip_address: IpAddr = "127.0.0.1".parse().expect("Invalid IP address");
    let port: u16 = 12060;
    let output_directory: &'static str = "C:/ftpup/N1MM/newfiles";

    // Create the output directory if it doesn't exist:
    if !fs::metadata(output_directory).is_ok() {
        fs::create_dir(output_directory).expect("Failed to create output directory");
    }

    // Bind to the specified IP address and port:
    let bind_address = SocketAddr::new(ip_address, port);
    let udp_socket = UdpSocket::bind(bind_address).expect("Failed to bind to socket");

    println!("Listening for UDP packets on {}:{}...", ip_address, port);

    // Create a buffer to store the dumped packets into:
    let mut buffer: [u8; BUFFER_SIZE] = [0u8; BUFFER_SIZE];

    // Main loop to receive and dump packets:
    loop {
        // Blocking receive UDP packet:
        match udp_socket.recv_from(&mut buffer) {
            Ok((size, source)) => {
                let packet_data: &[u8] = &buffer[..size];
                handle_packet(packet_data, output_directory, source);
            }
            Err(e) => {
                eprintln!("Error receiving packet: {}", e);
            }
        }
    }
}

/// Handles a packet.
/// 
/// This will serialise the `packet_data` received from the specified `source`
/// and dump it into a file in the specified `output_directory`.
fn handle_packet(packet_data: &[u8], output_directory: &str, source: SocketAddr) {
    // Construct the name of the file based on the source IP address, port, and
    // current system time since UNIX EPOCH in nano seconds:
    let file_name = format!(
        "packet_{}_{}_{}.dump",
        source.ip(),
        source.port(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos()
    );

    // Using the constructed file name, construct the file path by appending it
    // to the end of the provided output directory:
    let file_path = format!("{}/{}", output_directory, file_name);

    // Write the received packet bytes to the constructed file output directory:
    match fs::write(&file_path, packet_data) {
        Ok(_) => {
            // The write operation was successful:
            println!("Received packet from {}:{}", source.ip(), source.port());
            println!("Packet saved to: {}", file_path);
        }
        Err(e) => {
            // The write operation failed.
            eprintln
            !("Error saving packet to file: {}", e);
        }
    }
}