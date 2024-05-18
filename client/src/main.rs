use leptos::*;
use leptos_meta::provide_meta_context;
// use leptos_router::*;

mod pages;
mod data;
use crate::pages::chart::Example;
use leptos_server_signal::create_server_signal;
use serde::{Deserialize, Serialize};

use shared::types::*;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Provide websocket connection
    leptos_server_signal::provide_websocket("ws://localhost:5000/ws").unwrap();

    // Create server signal
    let event = create_server_signal::<WsEvent>("counter");

    view! { 
        <Example/>
        <h1>"Count: " {move || event.get().q_length.to_string()}</h1> 
    }
}


fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
