leptos_i18n::load_locales!();
use crate::i18n::*;
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};

async fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[component]
pub fn App() -> impl IntoView {
    let i18n = use_i18n();
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let new_msg = greet(&name).await;
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <main class="container">
            <h1>{t!(i18n, welcome)}{format!(" Zetta Project v{}", VERSION)}</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>
            <p>{ move || greet_msg.get() }</p>
        </main>
    }
}
