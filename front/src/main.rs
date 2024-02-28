// Import the Dioxus prelude to gain access to the `rsx!` macro and the `Scope` and `Element` types.
mod components;
pub mod models;

use shared::models::Film;
use components::{Header, Footer, FilmModal, FilmCard};
use dioxus::prelude::*;
use models::FilmModalVisibility;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    // Launch the web application using the App component as the root.
    dioxus_web::launch(App);
}

const API_ENDPOINT: &str = "api/v1";

fn films_endpoint() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let host = location.host().expect("should have a host");
    let protocol = location.protocol().expect("should have a protocol");
    let endpoint = format!("{}//{}/{}", protocol, host, API_ENDPOINT);
    format!("{}/films", endpoint)
}

async fn get_films() -> Vec<Film> {
    log::info!("Fetching films from {}", films_endpoint());
    reqwest::get(&films_endpoint())
        .await
        .unwrap()
        .json::<Vec<Film>>()
        .await
        .unwrap()
}

// Define a component that renders a div with the text "Hello, world!"
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || FilmModalVisibility(false));
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();
    let films = use_state::<Option<Vec<Film>>>(cx, || None);
    let selected_film = use_state::<Option<Vec<Film>>>(cx, || None);
    let force_get_films = use_state(cx, || ());

    {
        let films = films.clone();

        use_effect(cx, force_get_films, |_| async move {
            let existing_films = get_films().await;
            if existing_films.is_empty() {
                films.set(None);
            } else {
                films.set(Some(existing_films));
            }
        });
    }

    cx.render(rsx! {
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
                if let Some(films) = films.get() {
                    rsx!(
                        ul {
                            class: "flex flex-row justify-center items-stretch gap-4 flex-wrap",
                            {films.iter().map(|film| {
                                rsx!(
                                    FilmCard {
                                        key: "{film.id}",
                                        film: film,
                                        on_edit: move |_| {
                                            selected_film.get().clone();
                                            is_modal_visible.write().0 = true
                                        },
                                        on_delete: move |_| {}
                                    }
                                )
                            })}
                        }
                    )
                }
            }
            Footer{}
        }
        FilmModal {
            film: Option::<Film>::clone(&Option::None),
            on_create_or_update: move |_| {},
            on_cancel: move |_| {
                selected_film.set(None);
                is_modal_visible.write().0 = false;
            }
        }
    })
}
