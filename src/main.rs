mod server;
mod timeseries;

const HOST: &str = "127.0.0.1";
const PORT: i32 = 29191;

fn main() {
    let mut server = server::Server::new(HOST.to_string(), PORT);
    println!("Server starting on {}:{}", HOST, PORT);
    let run = server.run();
    if let Err(e) = run {
        panic!("Cannot start the server: {}", e.to_string());
    }
}
