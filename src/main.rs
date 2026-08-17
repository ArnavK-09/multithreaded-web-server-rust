use multithreaded_web_server_rust::Threadpool;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6967").unwrap();

    println!(
        "\nListening on: http://{}\n",
        listener.local_addr().unwrap()
    );

    let pool = Threadpool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let req = match buf_reader.lines().next() {
        Some(Ok(l)) => l,
        _ => return,
    };

    let (status_line, filename) = match &req[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let res = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(res.as_bytes()).unwrap();
}
