struct CryptoNode {
    ip: String,
    port: u16,
}

fn main() {
    let my_scan = CryptoNode {
        ip: String::from("127.0.0.1"),
        port: 443,
    };

    println!("Scanning {} on port {}", my_scan.ip, my_scan.port);
}
