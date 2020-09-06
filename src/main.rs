use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let server = Server::new("localhost:4100");
    server.run(WebsiteHandler);
}
