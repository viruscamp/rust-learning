//! https://www.zhihu.com/question/640220241

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::fmt::Write as _;

#[test]
fn main() {
    std::net::TcpListener::bind("127.0.0.1:8081")
        .and_then(|listener| {
            println!(
                "Starting server at {}",
                listener.local_addr()
                .map(|x| x.to_string())
                .unwrap_or(String::from("Unknown addr"))
            );
            Ok(listener)
        })
        .expect("cannot start server")
        .incoming()
        .filter_map(|stream| stream.ok())
        .map(|stream| {
            let stream = Arc::new(Mutex::new(stream));
            (
                {
                    let stream = stream.clone();
                    move || {
                        let mut buffer = [0; 1024];
                        stream.lock()
                            .expect("Cannot get lock when reading.")
                            .read(&mut buffer).and_then(|_|
                                Ok(String::from(String::from_utf8_lossy(&buffer))))
                    }
                },
                {
                    let stream = stream.clone();
                    move |string: &str| {
                        stream.lock()
                            .expect("Cannot get lock when writing.")
                            .write(&string.as_bytes())
                    }
                }
            )
        })
        .map(|(request, sender)|
            move || std::io::Result::Ok((request()?.split("\r\n")
                .map(|line|
                    line.split_at(line.find(": ").unwrap_or(0)
                ))
                .filter(|(_, value)| value.starts_with(": "))
                .map(|(key, value)| (key.to_string(), value[2..].to_string()))
                .collect::<BTreeMap<String, String>>(), sender))
        )
        .map(|process|
            || {
                let (header, sender) = process()?;
                let response = format!(
                    "<table border=\"1\" cellpadding=\"5\">{}</table>",
                    header.iter()
                        .map(|(key, value)| format!("<tr><td>{key}</td><td>{value}</td></tr>"))
                        .collect::<Vec<String>>()
                        .concat()
                );
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                    response.as_bytes().len(), response
                );
                sender(&response).map(|_| ()).unwrap_or_else(|error| {
                    println!("Error writing stream because {}", error.to_string());
                });
                std::io::Result::Ok(())
            }
        )
        .for_each(|process| {
            std::thread::spawn(process);
        });
}


/// 作者：古法皮卡丘
/// 链接：https://www.zhihu.com/question/640220241/answer/3369145682
/// 来源：知乎
/// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
#[test]
fn main2() {
    let listener = TcpListener::bind("127.0.0.1:8081").expect("cannot bind to address");
    if let Ok(addr) = listener.local_addr() {
        println!("Starting server at {addr}");
    } else {
        println!("Starting server at Unknown addr");
    }

    listener
        .incoming()
        .filter_map(Result::ok)
        .for_each(|stream| {
            std::thread::spawn(move || {
                handle_stream(stream).unwrap_or_else(|e| {
                    println!("Error handling stream: {e}");
                });
            });
        });
}

#[cfg(test)]
fn handle_stream(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec![0; 1024];
    stream.read(&mut buffer)?;
    let header = String::from_utf8_lossy(&buffer)
        .split("\r\n")
        .map(|line| line.split_at(line.find(": ").unwrap_or(0)))
        .filter(|(_, value)| value.starts_with(": "))
        .map(|(key, value)| (key.to_string(), value[2..].to_string()))
        .collect::<BTreeMap<_, _>>();

    let response_body = format!(
        "<table border=\"1\" cellpadding=\"5\">{}</table>",
        header
            .iter()
            .fold(String::new(), |mut output, (key, value)| {
                let _ = write!(output, "<tr><td>{key}</td><td>{value}</td></tr>");
                output
            })
    );
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        response_body.as_bytes().len(),
        response_body
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        println!("Error writing to stream: {e}");
        Err(e)
    } else {
        println!("Response sent: {response}");
        Ok(())
    }
}
