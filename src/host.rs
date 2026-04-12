use crate::html::document_from_body;
use crate::state::AppReference;
use axum::Form;
use axum::debug_handler;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use maud::html;
use serde::Deserialize;
use std::borrow::Cow;

pub(crate) const PASSWORD_COOKIE_NAME: &str = "host_password";

#[debug_handler]
pub async fn page(
    State(app): State<AppReference>,
    cookies: CookieJar,
) -> Result<(CookieJar, Html<String>), Redirect> {
    if !app.password_store.is_authorised(&cookies) {
        return Err(Redirect::to("/host/login"));
    };

    let players = app.players.lock().await;

    let document = document_from_body(html! {
        table {
            thead {
                tr {
                    th scope="col" { "Name" }
                    th scope="col" { "ID" }
                }
            }

            tbody {
                @for player in players.iter() {
                    tr {
                        td { (player.name) }
                        td { (player.id) }
                    }
                }
            }
        }
    });

    Ok((cookies, document))
}

#[debug_handler]
pub async fn login_page() -> Html<String> {
    document_from_body(html! {
        form action="/host/login" method="POST" {
            input name="password" type="password" {}
            input type="submit" {}
        }
    })
}

#[derive(Deserialize)]
pub(crate) struct LoginForm {
    password: Cow<'static, str>,
}

#[debug_handler]
pub async fn login(
    State(app): State<AppReference>,
    cookies: CookieJar,
    Form(form): Form<LoginForm>,
) -> Result<(CookieJar, Redirect), StatusCode> {
    if !app.password_store.is_correct(&form.password) {
        return Err(StatusCode::FORBIDDEN);
    }

    let new_cookies = cookies.add(
        Cookie::build((PASSWORD_COOKIE_NAME, form.password))
            .http_only(true)
            .secure(true),
    );

    Ok((new_cookies, Redirect::to("/host")))
}
