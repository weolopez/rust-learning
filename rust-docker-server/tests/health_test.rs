use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn spawn_server() -> Child {
    // Path to the compiled binary in the workspace target directory (debug build used by cargo test)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let bin_path = format!("{}/target/debug/rust-docker-server", manifest_dir);
    Command::new(bin_path)
        .spawn()
        .expect("failed to spawn rust-docker-server binary")
}

fn wait_for_listen(timeout: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
            let _ = stream.shutdown(std::net::Shutdown::Both);
            return true;
        }
        sleep(Duration::from_millis(100));
    }
    false
}

#[test]
fn health_endpoint_returns_ok() {
    let mut child = spawn_server();
    // wait up to 5s for server to start
    assert!(wait_for_listen(Duration::from_secs(5)), "server did not start in time");

    // Connect and request /health
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("connect failed");
    let req = b"GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    stream.write_all(req).expect("write failed");

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).expect("read failed");
    let resp = String::from_utf8_lossy(&buf);

    // Ensure response contains OK
    assert!(resp.contains("OK"), "unexpected response: {}", resp);

    // Cleanup
    let _ = child.kill();
    let _ = child.wait();
}
