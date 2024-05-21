use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use anyhow::{ Error, anyhow };
// use leptos_router::*;

mod types;
mod pages;
mod plots;
pub mod components;
use serde::{Deserialize, Serialize};

use shared::types::*;

use crate::pages::chartistry::Chartistry;
use crate::pages::event_stats::*;
use crate::pages::plotly::Plotly;
use crate::pages::mermaid::Mermaid;
use crate::pages::observable_plot::ObservablePlot;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Provide websocket connection
    leptos_server_signal::provide_websocket("ws://localhost:5000/ws").unwrap();

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

        <h2>Event Stats</h2>
        <EventStats/>

        // <h1>"Count: " {move || event.get().q_length.to_string()}</h1> 

    }
}


fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    // tracing_wasm::set_as_global_default_with_config(
    //     tracing_wasm::WASMLayerConfigBuilder::default()
    //         .set_max_level(tracing::Level::DEBUG)
    //         .build(),
    // );
    mount_to_body(|| view! { <App/> })
}
