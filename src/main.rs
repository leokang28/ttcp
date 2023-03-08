use std::io;
use tun_tap;
fn main() -> io::Result<()> {
    let _tap = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    let nbytes = _tap.recv(&mut buf)?;
    eprintln!("read {} bytes from {:x?}", nbytes, &buf[..nbytes]);
    Ok(())
}
