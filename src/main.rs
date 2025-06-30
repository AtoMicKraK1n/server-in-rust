use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;


mod balance;
use balance::{get_solana_balance, Cluster};

fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:9560").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    
    let (status_line, content_type, body) = match parse_request(request_line) {
        RequestType::Home => {
            let contents = fs::read_to_string("index.html").unwrap_or_else(|_| {
                String::from("<h1>Welcome to Solana Balance Server</h1>
                <p>Use /balance/{pubkey} to get account balance</p>
                <p>Example: /balance/11111111111111111111111111111112</p>")
            });
            ("HTTP/1.1 200 OK", "text/html", contents)
        }
        RequestType::Balance(pubkey) => {
            match get_solana_balance(&pubkey, Cluster::Devnet) {
                Ok(balance) => {
                    let json_response = format!(
                        "{{\"pubkey\":\"{}\",\"lamports\":{},\"sol\":{:.9}}}",
                        pubkey, balance.lamports, balance.sol
                    );
                    ("HTTP/1.1 200 OK", "application/json", json_response)
                }
                Err(err) => {
                    let error_response = format!(
                        "{{\"error\":\"{}\"}}",
                        err.error.replace("\"", "\\\"")
                    );
                    ("HTTP/1.1 400 BAD REQUEST", "application/json", error_response)
                }
            }
        }
        RequestType::NotFound => {
            let contents = fs::read_to_string("404.html").unwrap_or_else(|_| {
                String::from("404 - Not Found")
            });
            ("HTTP/1.1 404 NOT FOUND", "text/html", contents)
        }
    };

    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content_type,
        body.len(),
        body
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

enum RequestType {
    Home,
    Balance(String),
    NotFound,
}

fn parse_request(request_line: &str) -> RequestType {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    
    if parts.len() < 2 {
        return RequestType::NotFound;
    }

    let method = parts[0];
    let path = parts[1];

    if method != "GET" {
        return RequestType::NotFound;
    }

    if path == "/" {
        return RequestType::Home;
    }

    if path.starts_with("/balance/") {
        let pubkey = path.strip_prefix("/balance/").unwrap_or("");
        if !pubkey.is_empty() {
            return RequestType::Balance(pubkey.to_string());
        }
    }

    RequestType::NotFound
}
