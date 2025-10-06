use std::{
    fs,                            // for raeding files
    io::{BufReader, prelude::*},   // handling incoming data streams
    net::{TcpListener, TcpStream}, // server and client connection
    thread,                        // for multi-threading
};

fn main() {
    /*  Create two TCP listeners on localhost
        Port 8888 - serves web files on main server
        Port 7777 - redirects to http://google.com
    */
    let listener_8888 = TcpListener::bind("127.0.0.1:8888").unwrap();
    let listener_7777 = TcpListener::bind("127.0.0.1:7777").unwrap();

    /* spawn a new thread to handle port 8888 requests */
    thread::spawn(move || {
        for stream in listener_8888.incoming() {
            match stream {
                Ok(stream) => {
                    /* For each incoming TCP stream, spawn a new thread to handle */
                    thread::spawn(move || handle_8888(stream));
                }
                Err(e) => eprintln!("Error on port 8888: {}", e),
            }
        }
    });

    /* Handle port 7777 request. All requests are a 301 redirect to http://google.com */
    for stream in listener_7777.incoming() {
        match stream {
            Ok(mut stream) => {
                let response =
                    "HTTP/1.1 301 Moved Permanently\r\nLocation: http://www.google.com\r\n\r\n";
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Error writing redriect response: {}", e);
                }
            }
            Err(e) => eprintln!("Error on port 7777: {}", e),
        }
    }
}

/* Handle requests for port 8888.
   Requests are parsed for requested file
   and send HTTP response
*/
fn handle_8888(mut stream: TcpStream) {
    /* wrap the TCP stream in a buffered reader for easier line-by-line reading */
    let buf_reader = BufReader::new(&mut stream);

    /* read the first line of the HTTP request (GET request) */
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        /* if reading fails, stop */
        _ => return,
    };

    /* split request line */
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }

    /* remove leading '/' from the path
       default to "index.html" if the path is empty
    */
    let mut path = &parts[1][1..]; // skip the leading '/'
    if path.is_empty() {
        path = "index.html";
    }

    /* attempt to read the requested files from disk
       On success, respond with 200 OK and file's contents
       On failure, response with 404 Not Found
    */
    let (status_line, contents, content_type) = match fs::read(path) {
        Ok(contents) => ("HTTP/1.1 200 OK", contents, get_content_type(path)),
        Err(_) => {
            let not_found = b"<h1>404 Not Found</h1>".to_vec();
            ("HTTP/1.1 404 NOT FOUND", not_found, "text/html")
        }
    };

    // Build HTTP response header
    let length = contents.len();
    let header = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n"
    );

    // Send the HTTP response (header + file contents)
    if let Err(e) = stream.write_all(header.as_bytes()) {
        eprintln!("Error writing header: {}", e);
    }
    if let Err(e) = stream.write_all(&contents) {
        eprintln!("Error writing body: {}", e);
    }
}

/* determines content type based on file extension
   tells browser how to display */
fn get_content_type(filename: &str) -> &str {
    if filename.ends_with(".html") || filename.ends_with(".htm") {
        "text/html"
    } else if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
        "image/jpeg"
    } else if filename.ends_with(".png") {
        "image/png"
    } else if filename.ends_with(".mp4") {
        "video/mp4"
    } else if filename.ends_with(".mp3") {
        "audio/mpeg"
    } else if filename.ends_with(".ico") {
        "image/x-icon"
    } else {
        "text/plain"
    }
}
