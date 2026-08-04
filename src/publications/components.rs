use crate::publications::server::get_all_publications;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn ServerButton() -> impl IntoView {
    let request = Action::new(|_: &()| async move { get_all_publications().await });

    let pending = request.pending();
    let response = request.value();

    view! {
        <div>
            <button
                on:click=move |_| {
                    request.dispatch(());
                }
                disabled=move || pending.get()
            >
                {move || {
                    if pending.get() {
                        "Loading..."
                    } else {
                        "Call server"
                    }
                }}
            </button>

            <div>
                {move || {
                    match response.get() {
                        None => {
                            view! {
                                <p>"The server has not been called yet."</p>
                            }
                            .into_any()
                        }

                        Some(Ok(publications)) => {
                            view! {
                                <ul>
                                    {publications
                                        .into_iter()
                                        .map(|publication| {
                                            view! {
                                                <li>{publication}</li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            }
                            .into_any()
                        }

                        Some(Err(error)) => {
                            view! {
                                <p>{format!("Server error: {error}")}</p>
                            }
                            .into_any()
                        }
                    }
                }}
            </div>
        </div>
    }
}
