use std::io::{Read, Write}; // Import traits for reading from and writing to I/O streams
use std::net::TcpStream;    // Import a TCP client for raw socket connections

fn main() {
    // Collect all CLI args after the binary name into one space-joined String.
    // Similar to Java's String.join(" ", Arrays.copyOfRange(args, 0, args.length)).
    let t = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if t.is_empty() { return; } // If no text provided, exit quietly.

    // Percent-encode the text for use as a single query parameter.
    // This implements a minimal URL-encoding:
    // - Unreserved chars pass through (RFC 3986): A-Z a-z 0-9 - _ . ~
    // - Space becomes '+'
    // - All other bytes become %HH (uppercase hex)
    let mut q = String::with_capacity(t.len()); // Pre-allocate to avoid frequent reallocations
    for b in t.bytes() {
        match b {
            // Unreserved characters: append as-is
            b'A'..=b'Z'|b'a'..=b'z'|b'0'..=b'9'|b'-'|b'_'|b'.'|b'~' => q.push(b as char),
            // Encode space as '+'
            b' ' => q.push('+'),
            // Encode everything else as %HH (two hex nybbles)
            _ => {
                q.push('%');
                for h in [(b>>4)&0xF, b&0xF] {
                    q.push(if h < 10 { (b'0' + h) as char } else { (b'A' + (h - 10)) as char });
                }
            }
        }
    }

    // Compose a minimal HTTP/1.1 GET request with Connection: close
    // Note: HTTP requires CRLF line endings (\r\n)
    let req = format!(
        "GET /?text={} HTTP/1.1\r\nHost: localhost:8089\r\nConnection: close\r\n\r\n",
        q
    );

    // Open a TCP connection to 127.0.0.1:8089 (loopback)
    if let Ok(mut s) = TcpStream::connect(("127.0.0.1", 8089)) {
        // Send the request bytes to the server. Ignore errors for minimal output behavior.
        let _ = s.write_all(req.as_bytes());

        // Read the entire response into a buffer (headers + body).
        let mut buf = Vec::new();
        let _ = s.read_to_end(&mut buf);

        // Find the end of HTTP headers: the first "\r\n\r\n" sequence.
        // buf.windows(4) creates overlapping 4-byte slices to scan for the marker.
        if let Some(p) = buf.windows(4).position(|w| w == [b'\r', b'\n', b'\r', b'\n']) {
            // Write only the body (bytes after the header separator) to stdout.
            let _ = std::io::stdout().write_all(&buf[p + 4..]);
        }
    }
}