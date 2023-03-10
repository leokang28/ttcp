use std::io;
use tun_tap;
fn main() -> io::Result<()> {
    let _tap = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = _tap.recv(&mut buf)?;
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_protocal = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_protocal != 0x800 {
            // ipv4 only
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(header) => {
                let source_addr = header.source_addr();
                let dst = header.destination_addr();
                let protocal = header.protocol();
                let len = header.payload_len();
                let ttl = header.ttl();
                eprintln!(
                    "{} → {} {}bytes of protocal {} ttl={}",
                    source_addr,
                    dst,
                    len,
                    protocal,
                    ttl // eth_protocal,
                        // header
                );
            }
            Err(e) => {
                eprintln!("drop weird packet {:?}", e);
            }
        }
        // eprintln!(
        //     "read {} bytes(eth_flag:{:x} eth_protocal:{:x}) {:x?}",
        //     nbytes - 4,
        //     eth_flags,
        //     eth_protocal,
        //     header
        // );
    }
}
