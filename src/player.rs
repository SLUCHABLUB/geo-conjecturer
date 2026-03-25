use crate::Arguments;
use crate::html::document_from_body;
use axum::debug_handler;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Html;
use maud::Markup;
use maud::html;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct PageParameters {
    id: Option<Box<str>>,
    error: Option<Box<str>>,
}

#[debug_handler]
pub(crate) async fn page(
    State(arguments): State<&'static Arguments>,
    Query(parameters): Query<PageParameters>,
) -> Html<String> {
    document_from_body(html! {
        @if let Some(player_id) = parameters.id {
            (map(&player_id, arguments))
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

fn map(id: &str, arguments: &Arguments) -> Markup {
    let _ = id;

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
