use crate::publications::server::get_all_publications;
use futures_timer::Delay;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
#[component]
pub fn ServerButton() -> impl IntoView {
    let request = Action::new(|_: &()| async move {
        Delay::new(std::time::Duration::new(1, 0));
        get_all_publications().await
    });

    let pending = request.pending();
    let response = request.value();

    view! {
        <div class="space-y-4">
            <button
                class="btn btn-solid"
                on:click=move |_| {
                    request.dispatch(());
                }
                disabled=move || pending.get()
            >
                {move || { if pending.get() { "Loading..." } else { "Call server" } }}
            </button>

            <div>
                {move || {
                    match response.get() {
                        None => {
                            view! { <p class="muted">"The server has not been called yet."</p> }
                                .into_any()
                        }
                        Some(Ok(publications)) => {

                            view! {
                                <div class="table-wrap">
                                    <table class="data-table">
                                        <tr>
                                            <th>"title"</th>
                                            <th>"abs"</th>
                                            <th>"year"</th>
                                            <th>"journal"</th>
                                            <th>"link"</th>
                                        </tr>
                                        {publications
                                            .into_iter()
                                            .map(|publication| {
                                                view! {
                                                    <tr>
                                                        <td class="max-w-sm font-medium">{publication.title}</td>
                                                        <td class="muted max-w-md">{publication.abs}</td>
                                                        <td class="tabular-nums whitespace-nowrap">
                                                            {publication.year}
                                                        </td>
                                                        <td class="muted whitespace-nowrap">
                                                            {publication.journal}
                                                        </td>
                                                        <td>{publication.link}</td>
                                                    </tr>
                                                }
                                            })
                                            .collect_view()}
                                    </table>
                                </div>
                            }
                                .into_any()
                        }
                        Some(Err(error)) => {

                            view! {
                                <p class="notice notice-error">
                                    {format!("Server error: {error}")}
                                </p>
                            }
                                .into_any()
                        }
                    }
                }}
            </div>
        </div>
    }
}
