use server::Server;
use http::Request;
use http::Method;

mod http;
mod server;

fn main() {
    let address_and_port = String::from("127.0.0.1:8080").to_string();
    let server = Server::new(address_and_port);
    server.run(); 
}
