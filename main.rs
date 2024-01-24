use std::net::{TcpListener, TcpStream};
use std::result;
use std::fmt;
use std::thread;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};

const SAFE_MODE: bool = false;

struct Sensitive<T>(T);

impl<T: fmt::Display> fmt::Display for Sensitive<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(inner) = self; // destructuring
        if SAFE_MODE {
            writeln!(f, "[HIDDEN]")
        } else {
            writeln!(f, "{inner}")
        }
    }
}
enum Message {
    CLientConnected,
    ClientDisconneceted,
    NewMessage(Vec<u8>),
}
// we are storing the messages in the channels and then we are going to send them to the client
// server becomes our reciver
fn server(_message_receiver: Receiver<Message>) {
    todo!();
}
type Result<T> = result::Result<T, ()>;

// reading from the channel and sending it to the client
fn client( mut stream: TcpStream, messages:Sender<Message>) -> Result<()> {
    let _ = messages.send(Message::CLientConnected).map_err(|err| { // ack client connected
        eprintln!("ERROR: could not send the message to the server thread:  {err}");
    })?;
    let mut buffer = Vec::new();
    buffer.resize(1024,0);
    loop {
        let _n = stream.read(&mut buffer).map_err(|err| { 
            eprintln!("ERROR: could not read from the client: {err}");
            let _ = messages.send(Message::ClientDisconneceted);
        })?;
        let _ = messages.send(Message::ClientDisconneceted).map_err(|err| { 
            // if we cant send there is no channel anymore
            eprintln!("ERROR: could not send the message to the server thread: {err}"); 
        })?
    }
}

fn main() -> Result<()> {
    let address = "0.0.0.0:6969";
    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("Could not bind to address: {address}. Error: {err}", err = Sensitive(err));
    })?;
    println!("LOG: Listening to {}", Sensitive(address));
    let (message_sender, message_receiver) = channel();
    thread::spawn(|| server(message_receiver));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let message_sender = message_sender.clone();
                thread::spawn( ||client(stream, message_sender));
            }
            Err(err) => {
                eprintln!("ERROR: could not accept the incoming stream {err}");
            }
        }
    }
    Ok(())
}
