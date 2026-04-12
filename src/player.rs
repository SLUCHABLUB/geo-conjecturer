use crate::html::document_from_body;
use crate::state::AppReference;
use crate::state::MapDefaults;
use axum::Form;
use axum::debug_handler;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Html;
use axum::response::Redirect;
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
    State(app): State<AppReference>,
    Query(parameters): Query<PageParameters>,
) -> Html<String> {
    document_from_body(html! {
        @if let Some(player_id) = parameters.id {
            (map(&player_id, app.map_defaults))
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
        form action="/player/signup" method="POST" {
            input type="text" name="name" required {}
            input type="submit" value=("Play!") {}
        }
    }
}

fn map(id: &str, defaults: MapDefaults) -> Markup {
    let _ = id;

    html! {
        div
            id="map"
            style="height: 80vh"
            data-center={"[" (defaults.latitude) "," (defaults.longitude) "]"}
            data-zoom=(defaults.zoom)
        {
            div data-tile="OpenStreetMap" data-default-tile {}
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct SignupParameters {
    name: Box<str>,
}

#[debug_handler]
pub(crate) async fn signup(
    State(app): State<AppReference>,
    Form(parameters): Form<SignupParameters>,
) -> Redirect {
    let mut players = app.players.lock().await;

    let url = if let Some(uuid) = players.sign_up(&parameters.name) {
        format!("/player?id={uuid}")
    } else {
        format!(
            "/player?error=player named \"{}\" already exists",
            parameters.name
        )
    };

    Redirect::to(&url)
}
