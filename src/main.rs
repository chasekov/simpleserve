use simpleserve::server::SimpleServer;

fn main() {
    let mut server = SimpleServer::new("127.0.0.1:7878".to_string());
    server.start();

    println!("Shutting down.");
}
