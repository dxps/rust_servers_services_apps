use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let conn_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Listening on {}", self.socket_addr);

        // Listen to incoming connections in a loop.
        for stream in conn_listener.incoming() {
            let mut stream = stream.unwrap();
            let mut read_buffer = [0; 90];
            stream.read(&mut read_buffer).unwrap();
            // Convert incoming into our data structure.
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            println!("Got request {:?}", req);
            // And pass it to the router to pick the appropriate handler.
            Router::route(req, &mut stream);
        }
    }
}
