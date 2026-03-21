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
            html {
                head {
                    title {
                        "GeoConjecturer"
                    }
                }
                body {
                    "welcome"
                }
            }
        }
        .into_string(),
    )
}
