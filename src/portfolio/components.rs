use std::time::Duration;

use futures_timer::Delay;
use leptos::prelude::*;

use crate::portfolio::dto::PortfolioItemDto;
use crate::portfolio::server::get_all_portfolio_items;

#[component]
pub fn GithubRepoLoader() -> impl IntoView {
    let resource = Resource::new(
        || (),
        |_| async move {
            // Delay::new(Duration::new(1, 0));
            get_all_portfolio_items().await
        },
    );
    view! {
        <ErrorBoundary
         fallback=move|errors| format!("Error while loading gh: {:?}",errors.get()  )
        >
        <Suspense
        fallback=move || view! {<p class="muted"> "Loading o "  </p>}
        >
         {
            move || resource.and_then(move |comp| {
                let portfolio_items = comp.clone();
                view! {
                    <GithubRepoComponent portfolio_items/>
                }
            })
        }
        </Suspense>
        </ErrorBoundary>
    }
}

#[component]
fn GithubRepoComponent(portfolio_items: Vec<PortfolioItemDto>) -> impl IntoView {
    view! {
        <div class="table-wrap">
        <table class="data-table">
        <thead>
        <tr>
        <th> title </th>
        <th> description </th>
        <th> public </th>
        <th> public_url </th>
        <th> tags </th>
        </tr>
        </thead>
        <tbody>
        {
            portfolio_items.into_iter().map(|portfolio_item| view! { <PortfolioSingleComponentRow portfolio_item/> }).collect_view()
        }
        </tbody>
        </table>
        </div>
    }
}

#[component]
fn PortfolioSingleComponentRow(portfolio_item: PortfolioItemDto) -> impl IntoView {
    let tags_string = portfolio_item
        .tags
        .into_iter()
        .map(|item| format!("{} ", item.value))
        .collect::<String>();
    view! {
        <tr>
        <td class="font-medium whitespace-nowrap">{ portfolio_item.title } </td>
        <td class="max-w-md">{ portfolio_item.description } </td>
        <td class="muted">{ portfolio_item.public } </td>
        <td>{ portfolio_item.public_url } </td>
        <td>{tags_string} </td>
        </tr>
    }
}
