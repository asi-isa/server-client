use tokio;

mod server;
use server::run_server;

mod client;
use client::run_client;

mod args;
use args::get_args;

#[tokio::main]
async fn main() {
    let args = get_args();

    run_server(args.port).await;

    run_client().await;
}
