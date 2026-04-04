mod html;
mod player;
mod state;

use crate::state::AppReference;
use axum::Router;
use axum::debug_handler;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
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

    let app = AppReference::from_arguments(arguments);

    let router = Router::new()
        .route("/", get(root_page))
        .route("/player", get(player::page))
        .route("/player/signup", post(player::signup))
        .with_state(app);

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, arguments.port))
        .await
        .unwrap();

    serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn root_page() -> Redirect {
    Redirect::permanent("/player")
}
