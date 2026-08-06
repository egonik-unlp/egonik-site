use std::time::Duration;

use futures_timer::Delay;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::svg::view;

use crate::portfolio::dto::PortfolioItemDto;
use crate::portfolio::server::get_all_portfolio_items;

#[component]
pub fn GithubRepoLoader(count: RwSignal<i32>) -> impl IntoView {
    let resource = Resource::new(
        move || count.get(),
        |_| async move {
            Delay::new(Duration::from_secs(2)).await;
            get_all_portfolio_items().await
        },
    );
    view! {
            <Suspense fallback=move || {
                view! { <p class="muted">"Loading o "</p> }
            }>
                {move || {
            match resource.get() {
                None => view! {<p> "Nothing then there here cowboy"</p> }.into_any(),
                Some(data) => match data {

                Ok(portfolio_data) => {
                    let portfolio_items = portfolio_data.clone();

                    view! { <GithubRepoComponent portfolio_items /> }.into_any()
                }
                Err(errors) => {
                        view! { <div></div>     <p class="notice notice-error">
                                {format!("Error while loading GitHub repositories: {errors}")}
                            </p>
                        }.into_any()
                    }
                }

            }
                }}
            </Suspense>
    }
}

#[component]
fn GithubRepoComponent(portfolio_items: Vec<PortfolioItemDto>) -> impl IntoView {
    view! {
        <div class="table-wrap">
            <table class="data-table">
                <thead>
                    <tr>
                        <th>title</th>
                        <th>description</th>
                        <th>public</th>
                        <th>public_url</th>
                        <th>tags</th>
                    </tr>
                </thead>
                <tbody>
                    {portfolio_items
                        .into_iter()
                        .map(|portfolio_item| {
                            let tags_string = portfolio_item
                                .tags
                                .into_iter()
                                .map(|item| format!("{} ", item.value))
                                .collect::<String>();

                            view! {
                                <tr>
                                    <td class="font-medium whitespace-nowrap">
                                        {portfolio_item.title}
                                    </td>
                                    <td class="max-w-md">{portfolio_item.description}</td>
                                    <td class="muted">{portfolio_item.public}</td>
                                    <td>{portfolio_item.public_url}</td>
                                    <td>{tags_string}</td>
                                </tr>
                            }
                        })
                        .collect_view()}
                </tbody>
            </table>
        </div>
    }
}
