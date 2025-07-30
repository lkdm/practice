use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-leptos-ssr-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

// Server Function
#[server(UpdateCount, "")]
pub async fn update_count(increment_by: i64) -> Result<i64, ServerFnError> {
    println!("Count is {increment_by}");
    Ok(increment_by)
}

// Call from Component
#[component]
pub fn Counter() -> impl IntoView {
    let update_count = ServerAction::<UpdateCount>::new();
    let initial_increment = 1;

    view! {
            <ActionForm action=update_count>
                <span>Increment By:</span>
                <input type="number" name="increment_by" value=initial_increment/>
                <button type="submit">Update</button>
            </ActionForm>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Counter />
    }
}
