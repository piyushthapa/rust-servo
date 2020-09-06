use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("localhost:4100");
    server.run();
}
