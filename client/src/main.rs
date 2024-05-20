use leptos::*;
use leptos_meta::provide_meta_context;
use anyhow::{ Error, anyhow };
// use leptos_router::*;

mod pages;
mod plots;
mod data;
use leptos_server_signal::create_server_signal;
use serde::{Deserialize, Serialize};

use shared::types::*;

use crate::pages::chart::Chartistry;
use crate::pages::plotly::Plotly;
use crate::pages::mermaid::Mermaid;
use crate::pages::observable_plot::ObservablePlot;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Provide websocket connection
    // leptos_server_signal::provide_websocket("ws://localhost:5000/ws").unwrap();

    // Create server signal
    // let event = create_server_signal::<WsEvent>("counter");

    view! { 
        // <h2>Chartistry</h2>
        // <Chartistry/>

        // <h2>Plotly</h2>
        // <Plotly/>

        // <h2>Mermaid</h2>
        // <Mermaid/>

        // <h2>Observable Plot</h2>
        // <ObservablePlot/>

        // <h1>"Count: " {move || event.get().q_length.to_string()}</h1> 
    }
}


fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
