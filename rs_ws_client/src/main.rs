use log::*;
use tungstenite::{connect, Message, Result};
use url::Url;

fn read_mesage() -> Result<u32> {
    let (mut socket, response) =
        connect(Url::parse("ws://localhost:9001").unwrap()).expect("Can't connect");

    info!("Connected to the server");
    info!("Response HTTP code: {}", response.status());
    info!("Response contains the following headers:");

    socket
        .write_message(Message::Text("Hello WebSocket".into()))
        .unwrap();

    loop {
        let msg = socket.read_message().expect("Can't read a message");
        info!("uWS Received: {}", msg);
    }
}

fn main() {
    env_logger::init();

    let msg = read_mesage().unwrap();

    info!("Message : {}", msg);
}
