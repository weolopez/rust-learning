use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("rust-docker-server listening on http://0.0.0.0:8080");

    // Resolve static root directory. Priority:
    // 1. STATIC_ROOT env var
    // 2. ./rust-docker-server/static (when running from workspace root)
    // 3. ./static (current working directory)
    // 4. exe parent/parent/static
    let static_root: PathBuf = if let Ok(s) = std::env::var("STATIC_ROOT") {
        PathBuf::from(s)
    } else {
        let ws_path = PathBuf::from("./rust-docker-server/static");
        let cwd_path = PathBuf::from("./static");
        let exe_path = std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().and_then(|p| p.parent()).map(|pp| pp.join("static")));

        if ws_path.exists() {
            ws_path
        } else if cwd_path.exists() {
            cwd_path
        } else if let Some(ep) = exe_path {
            if ep.exists() {
                ep
            } else {
                PathBuf::from("./static")
            }
        } else {
            PathBuf::from("./static")
        }
    };

    println!("using static root: {}", static_root.display());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                    let sr = static_root.clone();
                    thread::spawn(move || {
                        if let Err(e) = handle_connection(stream, sr) {
                            eprintln!("connection error: {}", e);
                        }
                    });
            }
            Err(e) => eprintln!("accept error: {}", e),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, static_root: PathBuf) -> std::io::Result<()> {
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer)?;
    if n == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..n]);
    let first_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let path = if parts.len() >= 2 { parts[1] } else { "/" };

    println!("{} {}", parts.get(0).unwrap_or(&""), path);

    if path == "/health" {
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\n\r\nOK";
        stream.write_all(response.as_bytes())?;
        return Ok(());
    }

    let file_path = if path == "/" {
        static_root.join("index.html").to_string_lossy().to_string()
    } else {
        if path.contains("..") {
            let resp = "HTTP/1.1 403 Forbidden\r\nContent-Length: 9\r\nContent-Type: text/plain\r\n\r\nForbidden";
            stream.write_all(resp.as_bytes())?;
            return Ok(());
        }
    // trim leading slash and join with static_root
    let trimmed = path.trim_start_matches('/');
    static_root.join(trimmed).to_string_lossy().to_string()
    };

    let p = Path::new(&file_path);
    if p.exists() && p.is_file() {
        match File::open(&p) {
            Ok(mut f) => {
                let mut contents = Vec::new();
                f.read_to_end(&mut contents)?;
                let content_type = if file_path.ends_with(".html") { "text/html" }
                    else if file_path.ends_with(".css") { "text/css" }
                    else if file_path.ends_with(".js") { "application/javascript" }
                    else if file_path.ends_with(".png") { "image/png" }
                    else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") { "image/jpeg" }
                    else { "application/octet-stream" };

                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                    contents.len(), content_type
                );
                stream.write_all(header.as_bytes())?;
                stream.write_all(&contents)?;
            }
            Err(_) => {
                let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 21\r\nContent-Type: text/plain\r\n\r\nInternal Server Error";
                stream.write_all(response.as_bytes())?;
            }
        }
    } else {
        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\nContent-Type: text/plain\r\n\r\nNot Found";
        stream.write_all(response.as_bytes())?;
    }

    Ok(())
}
