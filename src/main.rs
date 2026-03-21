use axum::Router;
use axum::response::Html;
use axum::routing::get;
use maud::DOCTYPE;
use maud::html;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
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
                    {
                        div data-tile="OpenStreetMap" data-default-tile {}
                    }
                }
            }
        }
        .into_string()
    )
}
