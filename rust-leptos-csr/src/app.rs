use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <main class="bg-blue-400">
            <button on:click=move |_| set_count.set(count.get() + 1)>Click</button>
            <p class="text-red-400">{move || count.get()}</p>
        </main>
    }
}
