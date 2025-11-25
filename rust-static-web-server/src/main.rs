// Import the File type for reading files from the filesystem
use std::fs::File;
// Import Path for checking and manipulating filesystem paths
use std::path::Path;
// Import types from the tiny_http crate used to run a basic HTTP server
use tiny_http::{Server, Response, Header};

// Rust-focused notes for developers coming from Java/JavaScript/Python:
// - Ownership & borrowing: Rust enforces ownership rules at compile time. Values have a single owner
//   and you can borrow them with `&` (immutable) or `&mut` (mutable). This replaces a GC and
//   prevents many classes of runtime bugs, but it requires thinking about lifetimes.
// - Result<T, E> vs exceptions: Rust encodes recoverable errors as the Result enum. There are no
//   hidden exceptions; you must explicitly handle `Ok(...)` and `Err(...)` via `match`, `if let`,
//   or use the `?` operator to propagate errors up the call stack.
// - unwrap()/expect(): calling `unwrap()` on a Result will panic if it's an Err, which unwinds the
//   stack (similar to an uncaught exception). Prefer proper error handling (match/if let or `?`) in
//   libraries and only use unwrap/expect in quick prototypes or where a panic is acceptable.
// - Pattern matching: `match` is exhaustive and powerful for destructuring enums (Result, Option,
//   custom enums). It replaces many imperative try/catch patterns with clear, explicit branches.
// - Borrowed references (`&str`, `&Path`, etc.): methods that return references give you a borrow
//   tied to the lifetime of the source. You don't copy data unless you explicitly clone it.
// - Ignoring Results: using `let _ = request.respond(response);` intentionally drops the Result to
//   avoid unused-result warnings, but it also ignores potential errors. Better to log or handle them
//   when reliability matters.

// The program entry point
fn main() {
    // Create an HTTP server bound to 0.0.0.0:8080 so it is reachable from other machines/containers
    // The call returns a Result; unwrap will panic if binding fails
    let server = Server::http("0.0.0.0:8080").unwrap();
    // Print a startup message to stdout
    println!("Server started on http://0.0.0.0:8080");

    // Iterate over incoming HTTP requests; this blocks and yields each request as it arrives
    for request in server.incoming_requests() {
        // Extract the requested URL path (e.g. "/" or "/style.css")
        let url = request.url();
        // Log the HTTP method and URL to stdout for debugging
        println!("Received request: {} {}", request.method(), url);

        // Decide which file to serve based on the requested URL
        let file_path = if url == "/" {
            // Serve the default index file when the root path is requested
            "static/index.html".to_string()
        } else {
            // Security check: reject requests that attempt directory traversal
            if url.contains("..") {
                // Build a 403 Forbidden response if traversal is detected
                let response = Response::from_string("403 Forbidden").with_status_code(403);
                // Send the response to the client and ignore the result
                let _ = request.respond(response);
                // Skip further processing for this request
                continue;
            }
            // Map the URL path to a file under the static/ directory
            format!("static{}", url)
        };

        // Create a Path object for filesystem checks
        let path = Path::new(&file_path);

        // If the path exists and is a file, try to open and serve it
        if path.exists() && path.is_file() {
            match File::open(&path) {
                Ok(file) => {
                    // Choose a Content-Type header based on the file extension
                    let content_type = if file_path.ends_with(".html") {
                        "text/html"
                    } else if file_path.ends_with(".css") {
                        "text/css"
                    } else if file_path.ends_with(".js") {
                        "application/javascript"
                    } else if file_path.ends_with(".png") {
                        "image/png"
                    } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
                        "image/jpeg"
                    } else {
                        // Fallback content type for unknown extensions
                        "text/plain"
                    };

                    // Create a Content-Type header from bytes and unwrap the Result
                    let header = Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap();
                    // Build a response that streams the opened file and attach the header
                    let response = Response::from_file(file).with_header(header);
                    // Send the response to the client and ignore the result
                    let _ = request.respond(response);
                },
                Err(_) => {
                    // If the file could not be opened, return a 500 Internal Server Error
                    let response = Response::from_string("500 Internal Server Error").with_status_code(500);
                    let _ = request.respond(response);
                }
            }
        } else {
            // If the file does not exist, return a 404 Not Found
            let response = Response::from_string("404 Not Found").with_status_code(404);
            let _ = request.respond(response);
        }
    }
}