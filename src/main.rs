mod html;
mod player;

use axum::Router;
use axum::debug_handler;
use axum::response::Redirect;
use axum::routing::get;
use axum::serve;
use clap::Parser;
use dotenv::dotenv;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;

#[derive(Parser)]
struct Arguments {
    #[arg(short = 'N', long, env = "LATITUDE")]
    latitude: f32,
    #[arg(short = 'E', long, env = "LONGITUDE")]
    longitude: f32,
    #[arg(short = 'z', long, env = "ZOOM")]
    zoom: u8,

    #[arg(short, long, env = "PORT", default_value = "1337")]
    port: u16,
}

fn make_static<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

#[tokio::main]
async fn main() {
    let _ = dotenv();

    let arguments = make_static(Arguments::parse());

    let app = Router::new()
        .route("/", get(root_page))
        .route("/player", get(player::page))
        .with_state(arguments);

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, arguments.port))
        .await
        .unwrap();

    serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn root_page() -> Redirect {
    Redirect::permanent("/player")
}
