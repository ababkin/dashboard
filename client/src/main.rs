use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use anyhow::{ Error, anyhow };
// use leptos_router::*;
use leptonic::prelude::*;

mod types;
mod pages;
mod plots;
pub mod components;
use serde::{Deserialize, Serialize};

use shared::types::*;

use crate::pages::chartistry::Chartistry;
use crate::pages::data_chartistry::DataChartistry;
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
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Leptonic CSR template"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        <Title text="Leptonic CSR template"/>

        <Root default_theme=LeptonicTheme::default()>

            // <h2>Chartistry</h2>
            // <Chartistry/>

            // <h2>Plotly</h2>
            // <Plotly/>

            // <h2>Mermaid</h2>
            // <Mermaid/>

            // <h2>Observable Plot</h2>
            // <ObservablePlot/>

            <h2>Data fed chartistry</h2>
            <DataChartistry/>

            // <h1>"Count: " {move || event.get().q_length.to_string()}</h1> 
        </Root>

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
