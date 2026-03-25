use axum::Router;
use axum::debug_handler;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::get;
use axum::serve;
use clap::Parser;
use dotenv::dotenv;
use maud::DOCTYPE;
use maud::Markup;
use maud::html;
use serde::Deserialize;
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
        .route("/", get(root))
        .route("/player", get(player_page))
        .with_state(arguments);

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, arguments.port))
        .await
        .unwrap();

    serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn root() -> Redirect {
    Redirect::permanent("/player")
}

fn document_from_body(body: Markup) -> Html<String> {
    let markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8"
                meta name="viewport" content="width=device-width, initial-scale=1.0"

                title {
                    "GeoConjecturer"
                }

                link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css";
                script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" {}
                script src="https://www.unpkg.com/hyperleaflet@0.5.1/dist/hyperleaflet.umd.js" {}
            }
            body hx-boost=(true) {
                (body)
            }
        }
    };

    Html(markup.into_string())
}

#[derive(Deserialize)]
struct PlayerPageParameters {
    id: Option<Box<str>>,
    error: Option<Box<str>>,
}

#[debug_handler]
async fn player_page(
    State(arguments): State<&'static Arguments>,
    Query(parameters): Query<PlayerPageParameters>,
) -> Html<String> {
    document_from_body(html! {
        @if let Some(player_id) = parameters.id {
            (player_map(&player_id, arguments))
        } @else {
            @if let Some(error) = parameters.error {
                p { (error) }
            }

            (name_input())
        }
    })
}

fn name_input() -> Markup {
    html! {
        form action="/player/signup" method="GET" {
            input type="text" name="name" required {}
            input type="submit" value=("Play!") {}
        }
    }
}

fn player_map(player_id: &str, arguments: &Arguments) -> Markup {
    let _ = player_id;

    html! {
        div
            id="map"
            style="height: 80vh"
            data-center={"[" (arguments.latitude) "," (arguments.longitude) "]"}
            data-zoom=(arguments.zoom)
        {
            div data-tile="OpenStreetMap" data-default-tile {}
        }
    }
}
