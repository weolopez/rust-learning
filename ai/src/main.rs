use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let t = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if t.is_empty() { return; }

    // inline percent-encode
    let mut q = String::with_capacity(t.len());
    for b in t.bytes() {
        match b {
            b'A'..=b'Z'|b'a'..=b'z'|b'0'..=b'9'|b'-'|b'_'|b'.'|b'~' => q.push(b as char),
            b' ' => q.push('+'),
            _ => { q.push('%'); for h in [(b>>4)&0xF, b&0xF] { q.push(if h<10 {(b'0'+h) as char} else {(b'A'+(h-10)) as char}); } }
        }
    }

    let req = format!("GET /?text={} HTTP/1.1\r\nHost: localhost:8089\r\nConnection: close\r\n\r\n", q);

    if let Ok(mut s) = TcpStream::connect(("127.0.0.1", 8089)) {
        let _ = s.write_all(req.as_bytes());
        let mut buf = Vec::new();
        let _ = s.read_to_end(&mut buf);
        if let Some(p) = buf.windows(4).position(|w| w==[b'\r',b'\n',b'\r',b'\n']) {
            let _ = std::io::stdout().write_all(&buf[p+4..]);
        }
    }
}
