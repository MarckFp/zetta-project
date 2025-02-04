mod app;

use app::*;
use leptos::prelude::*;
use crate::i18n::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <I18nContextProvider>
                <App/>
            </I18nContextProvider>
        }
    })
}
