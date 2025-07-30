use leptos::{either::Either, prelude::*};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
            </head>
            <body>
                <PostApp/>
            </body>
        </html>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    id: u16,
    name: String,
}

#[cfg(features = "ssr")]
pub mod ssr {
    pub async fn db() -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "ssr")]
pub mod ssr {
    // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use leptos::server_fn::ServerFnError;
    use sqlx::{Connection, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:Todos.db").await?)
    }
}

#[component]
pub fn PostApp() -> impl IntoView {
    view! {
        <header>
            <h1>"My posts"</h1>
        </header>
        <main>
            <Posts/>
        </main>
    }
}

#[component]
pub fn Posts() -> impl IntoView {
    view! {
        <div>Posts</div>
    }
}
