use std::net::TcpListener;
use std::result;
use std::io::Write;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    let address = "127.0.0.1:6969";

    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("Could not bind to address: {address}. Error: {err}")
    })?;

    println!("LOG: Listening to {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                writeln!(stream, "Hello, mein Bruder !").map_err(|err| {
                    eprintln!("ERROR: during writing to the user {err}");
                })?;
            }
            Err(err) => {
                eprintln!("ERROR: could not accept the incoming stream {err}");
            }
        }
    }
    Ok(())
}
