use axum::Router;
use axum::response::Html;
use axum::routing::get;
use clap::Parser;
use dotenv::dotenv;
use maud::DOCTYPE;
use maud::html;
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

    let app = Router::new().route("/", get(|| root(arguments)));

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, arguments.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root(arguments: &Arguments) -> Html<String> {
    Html(
        html! {
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
                body {
                    div
                        id="map"
                        style="height: 100vh"
                        data-center={"[" (arguments.latitude) "," (arguments.longitude) "]"}
                        data-zoom=(arguments.zoom)
                    {
                        div data-tile="OpenStreetMap" data-default-tile {}
                    }
                }
            }
        }
        .into_string()
    )
}
