use axum::response::Html;
use maud::DOCTYPE;
use maud::Markup;
use maud::html;

pub(crate) fn document_from_body(body: Markup) -> Html<String> {
    let markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8"
                meta name="viewport" content="width=device-width, initial-scale=1.0"

                title {
                    "GeoConjecturer"
                }

                // TODO: Inline or self-host these.
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
