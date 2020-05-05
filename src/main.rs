use std::io;

use tokio::codec::{BytesCodec, Decoder};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::io::ErrorKind;

use bytes::Bytes;

use hex_slice::AsHex;
use std::time::Duration;
use chrono;

fn valid_request(mb_tcp_frame : &bytes::Bytes) -> std::result::Result<(), std::io::Error>
{
    let len = mb_tcp_frame.len();
    if len != 12
    {
        return Err(io::Error::new(ErrorKind::InvalidData, "Len is Incorrect"));
    }

    return Ok(());
}

fn main()
{
    println!("TCP Server");

    let addr = match format!("{}:{}", "0.0.0.0", 502).parse()
    {
        Ok(addr) => {addr},
        Err(_) => {
            println!("Incorrect IP-ADDRESS or TCP-PORT");
            std::process::exit(1);
        }
    };

    let listener = match TcpListener::bind(&addr)
    {
        Ok(listener) => {listener},
        Err(e) => {
            println!("Error. Cannot bind. Message: {}", e.to_string());
            std::process::exit(1);
        }
    };

    println!("listening...{:?}", listener);

    let done = listener.incoming().map_err(move |e| {
	 println!("Error: {}", e.to_string());
	}).for_each(move |socket| {
	       socket.set_nodelay(true).expect("Cannot set no delay option.");
           socket.set_keepalive(Some(Duration::from_secs(1))).expect("Cannot set keepalive option.");

           let remote = socket.peer_addr().expect("Cannot resolve TCP remote.").to_string();
           println!("Conected: {}", remote);

           let framed = BytesCodec::new().framed(socket);
           let (_tcp_writer, tcp_reader) = framed.split();

           let processor = tcp_reader.for_each(
               move |bytes|{
                   println!("TCP Payload: {:02X} len: {}", bytes.as_hex(), bytes.len());
                   let tcp_data = Bytes::from(bytes.clone());
                   valid_request(&tcp_data)?;
                   Ok(())
               }).and_then(move |()| {
                   Ok(())
               }).or_else(move |err| {
	                  println!("{} Error: {}", chrono::offset::Local::now(), err);
                      Err(err)
                  }).then(move |_result| {
                      Ok(())
                  });

         tokio::spawn(processor)
     });

    tokio::run(done);
}
