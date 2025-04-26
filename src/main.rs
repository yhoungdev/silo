use std::net::SocketAddr;
use axum::Server;
mod routes;

#[tokio::main]
async fn main() {

    let app = routes::routes();
    let port = 3000;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("listening on {}" , addr);

    tokio::net::TcpListener::bind(&addr)

}