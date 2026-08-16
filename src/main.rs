use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6967").unwrap();

    print!("\x1B[2J\x1B[1;1H");
    println!(
        "\nListening on: http://{}\n",
        listener.local_addr().unwrap()
    );

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established...");
        dbg!(stream);
    }
}
