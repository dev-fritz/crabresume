use std::net::TcpListener;

use crabresume::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:8000").expect("Failed on bind port.");
    run(listener)?.await
}
